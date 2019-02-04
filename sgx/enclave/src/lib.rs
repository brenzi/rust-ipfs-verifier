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

#![crate_name = "IpfsVerifierEnclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]


extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tcrypto;

//extern crate multihash;
//extern crate cid;
extern crate rust_base58;
//extern crate sha2;

use sgx_types::*;
use std::prelude::v1::*;
//use std::string::String;
//use std::vec::Vec;
//use std::io::{self, Write};
use std::slice;

use sgx_tcrypto::*;

//use rust_base58::{ToBase58, FromBase58};
// use sha2::{Sha256, Digest};
use rust_base58::{ToBase58, FromBase58};
// use multihash::{encode, decode, Hash, Multihash, to_hex};
// use cid::{Cid, Codec, Version, Prefix};

#[no_mangle]
pub extern "C" fn cid_verify(data_string: *const u8, data_len: usize, cid_string: *const u8, cid_len: usize) -> sgx_status_t {

    let data_slice = unsafe { slice::from_raw_parts(data_string, data_len) };
    let cid_slice = unsafe { slice::from_raw_parts(cid_string, cid_len) };
/*    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();
 */
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
    let cid = ToBase58::to_base58(&output[..]);
    let cid = output;
    //let h = multihash::encode(multihash::Hash::SHA2256, data_slice).unwrap();
    //let cid = Cid::new(Codec::Raw, Version::V1, &h);

    println!("SGX: input cid: {:?} ", cid_slice);
    println!("SGX: true  cid: {:?} ", cid);
    // Ocall to normal world for output
    // println!("{}", &hello_string);

    sgx_status_t::SGX_SUCCESS
}