# (trusted) rust-ipfs-verifier
verify ipfs content matches multihash within Intel SGX enclave

based on [rust-sgx-sdk](https://github.com/baidu/rust-sgx-sdk)
## prerequisites
* Intel SGX enabled hardware.
* Intel SGX enabled in bios
* [install intel driver V.2.4](https://download.01.org/intel-sgx/linux-2.4/docs/Intel_SGX_Installation_Guide_Linux_2.4_Open_Source.pdf)

### quick version for ubuntu 16.04
```
> sudo apt install libssl-dev libcurl4-openssl-dev libprotobuf-dev build-essential python alien
> wget https://download.01.org/intel-sgx/linux-2.4/ubuntu16.04-server/sgx_linux_x64_driver_778dd1f.bin
> wget https://download.01.org/intel-sgx/linux-2.4/ubuntu16.04-server/libsgx-enclave-common_2.4.100.48163-xenial1_amd64.deb
> wget https://download.01.org/intel-sgx/linux-2.4/ubuntu16.04-server/sgx_linux_x64_sdk_2.4.100.48163.bin
> wget http://registrationcenter-download.intel.com/akdlm/irc_nas/11414/iclsClient-1.45.449.12-1.x86_64.rpm
> sudo alien --scripts iclsClient-1.45.449.12-1.x86_64.rpm
> sudo dpkg -i iclsclient_1.45.449.12-2_amd64.deb
# re-do the following command after every kernel update!
> sudo ./sgx_linux_x64_driver.bin
> sudo dpkg -i ./libsgx-enclave-common_2.4.100.48163-xenial1_amd64.deb
> sudo ./sgx_linux_x64_sdk_2.4.100.48163.bin
```
install the sdk to /opt/intel

now check installation
```
> lsmod | grep sgx
isgx                   49152  1
> sudo service aesmd start
```

### running in SW simulation mode
If you don't have an SGX enabled machine, you can still run in simulation mode:

```
> wget https://download.01.org/intel-sgx/linux-2.4/ubuntu16.04-server/sgx_linux_x64_sdk_2.4.100.48163.bin
> wget https://download.01.org/intel-sgx/linux-2.4/ubuntu16.04-server/libsgx-enclave-common_2.4.100.48163-xenial1_amd64.deb
> sudo dpkg -i ./libsgx-enclave-common_2.4.100.48163-xenial1_amd64.deb
```
install the sdk to /opt/intel


##
to run in HW mode: 
```
> make
> cd bin
> ./app
```

to run in SW mode (tested on WSL)
```
> export SGX_MODE=SW
> export LD_LIBRARY_PATH=/opt/intel/sgxsdk/lib64/
> make
> cd bin
> ./app
```