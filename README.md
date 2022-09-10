# RISC Zero Multi Payment Hashes

`risc0-mph` allows you to create a prove that you own preimages that add to up to a single preimage. This can be used to allow lightning payments to be linked to a single preimage to unlock an onchain payment.


The guest(zkp program) can be found at [`./methods/guest/src/bin/mph.rs`](./methods/guest/src/bin/mph.rs).

The starter(executer/verifier) can be found at [`./starter/src/main.rs`](./starter/src/main.rs).

## Running the prover
- install the [nix package manager](https://nixos.org/download.html)
- run `nix-shell`
- run `cargo run --release`


example output:
```
preimage 0: b8d58d22978a16f394b60b3c8b867aca63adbb221cd97183387c8191a5971966
preimage 1: e69ed89f3ced8049771fcecee92d9f2b9cafc18c4d878c6bceb58740099dcc82
preimage 2: b7ac5514378c9e4666fd4eccd7e470b6c252149a67f5ce951862e57f229a20ae
payment hash 0: d5ea3c403291bd1cd7a20a4980b66471729e4f5363e46ea9735131176cde6acf
payment hash 1: 9e5076a26f1752b9780cdf9cb9092c6208b1fdae5e98f2d3d164e2d95a458fb3
payment hash 2: e5f592ff29c107be2e89efe50aa4b4dc577ae593294bb9b88d145b59bd562325
final preimage: e2d0173725162cf5c548f534320037c31f00214f25c6dd2bcfe34bef023144ac
final payment hash: 980312f5dcfb31d9b9baa77fa3e000f9e62c6aa327bb04878e54821869ff4ccc
size of prove: 217576 bytes
Total time taken to run is 15894ms
```