[crates]: https://crates.io/crates/mortem
[docs]: https://docs.rs/mortem

# Mortem
[![Crates.io](https://img.shields.io/crates/v/mortem)][crates]
[![docs.rs](https://img.shields.io/docsrs/mortem)][docs]

Easy self-deleting executable.

### Usage
Simply register a guard (either `soft` or `hard`) in the program entrypoint, and have it be dropped to delete the binary.
```rust
fn main() {
    let _mortem = mortem::hard(); // register mortem guard

    // some code
    println!("Hello!")

    // _mortem drops and executable is deleted
}
```

### What's with `soft` and `hard`?
The `soft` handler exits on IO errors and only tries to delete the executable once;
the `hard` handler keeps trying till the executable is successfully deleted.

### Examples
See the [documentation](docs).