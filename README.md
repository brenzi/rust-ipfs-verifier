# rust-ipfs-verifier
verify ipfs content matches multihash

verifies multihash based on content, in order to be sure ipfs address and contant are a match
```
> echo awesome test content > test.txt
> ipfs add --raw-leaves test.txt
zb2rhgCbaGmTcdZVRpZi3Z8CsdtAbFv7PRdRD9s6mKtef6LK9
> ipfs add --hash blake2b-256 test.txt
zCT5htke82ziqES2sZUP6MPR1EcC3DchZjbFuGeQcTfm16m5q5e8
> curl --form "fileupload=@test.txt" http://127.0.0.1:5001/api/v0/add?raw-leaves=True
{"Name":"test.txt","Hash":"zb2rhgCbaGmTcdZVRpZi3Z8CsdtAbFv7PRdRD9s6mKtef6LK9","Size":"21"}
```
These addresses are reproduced based on the string `awesome test content\n`

1. retrieve content from ipfs
2. compute multihash from content
3. verify matching multihashes (retrieved address and computed multihash)
