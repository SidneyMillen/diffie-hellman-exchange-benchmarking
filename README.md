This is a small rust program to benchmark various forms of Diffie-Hellman Key Exchange with the [x25519-dalek library](https://github.com/dalek-cryptography/curve25519-dalek) for a Cryptography class project

The benchmark is performed using cargo nightly's cargo bench subcommand, the main function in the program does nothing useful beyond verifying the mutual secret in n-party exchange is truly shared by all parties.
