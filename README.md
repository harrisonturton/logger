## logger

A very simple logger that prints in color. Uses the facade provided by the
[`log`](https://crates.io/crates/log) crate.

## Usage

Requires the `log` crate to be added as a dependency.

```rust
pub fn main() {
    logger::init().unwrap();
    log::info!("This is an informational log");
    log::error!("This is an error log");
}
``` 
