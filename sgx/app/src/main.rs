//  Copyright (c) 2019 Alain Brenzikofer
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

extern crate futures;
extern crate hyper;
extern crate ipfs_api;

extern crate sgx_types;
extern crate sgx_urts;
extern crate dirs;
use sgx_types::*;
use sgx_urts::SgxEnclave;

use std::io::{Read, Write, Cursor};
use std::fs;
use std::path;

use futures::Future;
use ipfs_api::IpfsClient;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";
static ENCLAVE_TOKEN: &'static str = "enclave.token";

extern {
    fn cid_verify(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
                     data_string: *const u8, data_len: usize,
                     cid_string: *const u8, cid_len: usize) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {

    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // Step 1: try to retrieve the launch token saved by last transaction
    //         if there is no token, then create a new one.
    //
    // try to get the token saved in $HOME */
    let mut home_dir = path::PathBuf::new();
    let use_token = match dirs::home_dir() {
        Some(path) => {
            println!("[+] Home dir is {}", path.display());
            home_dir = path;
            true
        },
        None => {
            println!("[-] Cannot get home dir");
            false
        }
    };

    let token_file: path::PathBuf = home_dir.join(ENCLAVE_TOKEN);;
    if use_token == true {
        match fs::File::open(&token_file) {
            Err(_) => {
                println!("[-] Open token file {} error! Will create one.", token_file.as_path().to_str().unwrap());
            },
            Ok(mut f) => {
                println!("[+] Open token file success! ");
                match f.read(&mut launch_token) {
                    Ok(1024) => {
                        println!("[+] Token file valid!");
                    },
                    _ => println!("[+] Token file invalid, will create new token file"),
                }
            }
        }
    }

    // Step 2: call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    let enclave = try!(SgxEnclave::create(ENCLAVE_FILE,
                                          debug,
                                          &mut launch_token,
                                          &mut launch_token_updated,
                                          &mut misc_attr));

    // Step 3: save the launch token if it is updated
    if use_token == true && launch_token_updated != 0 {
        // reopen the file with write capablity
        match fs::File::create(&token_file) {
            Ok(mut f) => {
                match f.write_all(&launch_token) {
                    Ok(()) => println!("[+] Saved updated launch token!"),
                    Err(_) => println!("[-] Failed to save updated launch token!"),
                }
            },
            Err(_) => {
                println!("[-] Failed to save updated enclave token, but doesn't matter");
            },
        }
    }

    Ok(enclave)
}

fn main() {

    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    // 
    println!("connecting to localhost:5001...");
    let client = IpfsClient::default();
    // write data to ipfs
    let data = b"awesome test content\n";
    let datac = Cursor::new(data);
    let req = client
        .add(datac)
        .map(|res| {
            println!("{}", res.hash);
            //let addrmh = decode(res.hash.as_bytes()).unwrap();  
            //let hash = addrmh.digest;   
            //println!("digest: {}", str::from_utf8(hash).unwrap())
            //println!("digest: {:?}", to_hex(hash))
        })
        .map_err(|e| eprintln!("{}", e));

    hyper::rt::run(req);
    // FIXME: if ipfs-api would support --raw-leaves, the address would be

/*
    // first compute the thing in untrusted app (easier debugging)

    // hardcoded multihash
    let size: u8 = 32;      //256bit=32bytes
    let code: u8 = 0x12;    // SHA2256
    //let code = 0x41;    // Blake2s
    let mut output = Vec::new();
    //output.resize(2 + size as usize, 0);
    output.push(code);
    output.push(size);
    let hash = rsgx_sha256_slice(data_slice).unwrap();
    // FIXME: concatenate must be easier than this!
    for h in hash.iter() {
        output.push(*h);
    }


    //let h = multihash::encode(multihash::Hash::SHA2256, data_slice).unwrap();
    //let cid = Cid::new(Codec::Raw, Version::V1, &h);

    println!("SGX: input cid: {:?} ", cid_slice);
    println!("SGX: true  cid: {:?} ", output);

*/


    //let data_string = String::from_(data);
    let cid_string = String::from("zb2rhgCbaGmTcdZVRpZi3Z8CsdtAbFv7PRdRD9s6mKtef6LK9\n");

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let result = unsafe {
        cid_verify(enclave.geteid(),
                      &mut retval,
                      data.as_ptr() as * const u8,
                      data.len(),
                      cid_string.as_ptr() as * const u8,
                      cid_string.len())
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    println!("[+] cid_verify success...");

    enclave.destroy();
}
