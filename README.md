# zkvm-blockchain-prover

**This project is for educational purposes only and is not intended for production use.**

This project demonstrates a blockchain prototype integrated with [RISC Zero zkVM](https://github.com/risc0) to prove the validity and consistency of the blockchain state. The blockchain state is computed on the **host machine** and validated in a **zkVM guest** environment. The host program proves the blockchain’s validity to any verifier, leveraging zero-knowledge proofs. Make sure you have the Rust toolchain (1.75+), `cargo` for building and running, and the `risc0` toolchain (installed via `cargo install cargo-risczero` if needed).

## Quick Development: Leveraging dev-mode

During the development of your project, running your code can take a long time due to the proof generation process. To address this issue and allow for faster iterations of your code, we suggest utilizing dev-mode. This mode bypasses the time-consuming proof generation process. To activate dev-mode, set the environment variable `RISC0_DEV_MODE=1` when executing your project:  
`RISC0_DEV_MODE=1 cargo run --release`

## Real Proof Generation

Once you've reached a point where you're ready to generate real proofs, you can do so by setting `RISC0_DEV_MODE=0`. Generating proofs locally would be achieved by running the following:  
`RISC0_DEV_MODE=0 cargo run --release`

Note that since proofs are now being generated, the execution time will be significantly longer than when running in dev-mode. To create a proof with the zkVM on your own machine, we recommend at least 16 GB of RAM. To avoid these hardware requirements, consider using [Bonsai](https://bonsai.xyz) to generate proofs remotely, as it will be significantly faster than running proofs locally. You can request access to Bonsai to set additional flags.

## Executor Statistics

To gain insights into your application's performance, you can obtain executor statistics by setting the `RUST_LOG` environment variable to `info`. Setting this filter will print statistics about the execution before proof generation, so you can understand how computationally expensive your application is. Since the statistics concern only the executor phase, it is recommended to run your application in dev-mode to avoid the overhead of proof generation:  
`RISC0_DEV_MODE=1 RUST_LOG=info RISC0_INFO=1 cargo run --release`

## License

MIT
