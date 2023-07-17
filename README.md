# smolmask

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/smolmask.svg
[crates-url]: https://crates.io/crates/smolmask
[mit-badge]: https://img.shields.io/badge/license-mit-blue.svg
[mit-url]: https://github.com/drbh/smolmask/blob/master/LICENSE

Boolean arrays compressed into integers. Useful for storing boolean arrays in databases with just a single integer.

```rust
use smolmask::BoolArray;

fn main() {
    let bools = vec![true, false, true, true];
    let integer: u64 = BoolArray::store(&bools).unwrap();
    println!("Integer: {}", integer); // 1096635
    println!("Length: {}", BoolArray::length(integer)); // 4
    println!("{:?}", BoolArray::retrieve(integer)); // [true, false, true, true]
}
```
