# library_sim

A small Rust library simulation used for learning Rust and practicing basic ownership/borrowing and simple program structure.

This project contains a minimal command-line simulation of a library with:

- `Book` structs (title, author, pages, available)
- `Patron` structs (name, id)
- `Library` with methods to add/display/checkout/return books and query availability
- Simple daily action simulation (checkout / return / browse)

## Quick start

Requirements
- Rust and Cargo (stable toolchain)

Build and run

```bash
cd library_sim
cargo build
cargo run
```

The program prints the library catalog, simulates several days of user actions (checkouts, returns, browsing), and prints availability statistics.

## Project layout

- `Cargo.toml` — Cargo manifest
- `src/main.rs` — main program and core logic

## How to use

- Add books using `Library::add_book()` or by editing `src/main.rs` sample data.
- Run `cargo run` to execute the simulation. Output includes daily actions and final availability counts.

## Contributing

Small fixes and improvements welcome. Suggested tasks:
- Add tests for `Library` methods
- Improve random action weighting
- Add CLI options for number of days, seed, or input file

## License

This repository is provided for learning. Add a license file if you plan to publish (for example, `LICENSE` with MIT or Apache-2.0).
