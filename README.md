# rs-x11-hash
Rust bindings for x11 hash
> Performs the [x11 hashing](https://docs.dash.org/en/latest/introduction/features.html#x11-hash-algorithm) algorithm used in the [Dash cryptocurrency](https://dash.org) in JavaScript.

x11 hashing algorithm sequentially uses:

```
• BLAKE
• BLUE MIDNIGHT WISH (BMW)
• Groestl
• JH
• Keccak (An algorithm whose variant gave rise to SHA-3)
• Skein
• Luffa
• CubeHash
• SHavite-3
• SIMD
• ECHO
```

C-sources taken from [DashSync](https://github.com/dashpay/dashsync-iOS/tree/master/DashSync/shared/crypto/x11)

## Usage

Install the library as a Cargo crate.

```
$ cargo install rs-x11-hash
```
or use as a dependency in Cargo.toml

```
[dependencies]
rs-x11-hash = "0.1.4"
```

Reference the library

```rust
use hex::{FromHex, ToHex};
let x11 = "020000002cc0081be5039a54b686d24d5d8747ee9770d9973ec1ace02e5c0500000000008d7139724b11c52995db4370284c998b9114154b120ad3486f1a360a1d4253d310d40e55b8f70a1be8e32300";
let x11_vec = Vec::from_hex(x11).unwrap();
let md = rs_x11_hash::get_x11_hash(x11_vec);
println!("input: {}", x11);
println!("output: {:?}", md.encode_hex::<String>());
assert_eq!(md.to_vec(), Vec::from_hex("f29c0f286fd8071669286c6987eb941181134ff5f3978bf89f34070000000000").unwrap())
```
