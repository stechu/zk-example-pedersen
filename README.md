# zk-example

This repo contained two examples of using arkworks zkSNARK to build two simple circuits:

* pedersen hash example (CRH):
    ```bash
    cargo build --release -p pedersen
    cargo run --bin pedersen
    ```

* blake2s hash example (PRF):
    ```bash
    cargo build --release -p prf
    cargo run --bin prf
    ```
