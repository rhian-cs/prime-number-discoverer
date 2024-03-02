## Installation

Dependencies:

- Rust and Cargo (https://rustup.rs/)
- sqlite3 (Ubuntu: `sudo apt install sqlite3`)

Clone the repository and run:

```rs
cargo run --release
```

All the found prime numbers can be found in the `tmp/primes.db` database file, inside the `primes` table.
