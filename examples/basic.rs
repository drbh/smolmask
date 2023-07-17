use smolmask::BoolArray;

fn main() {
    let bools = vec![
        true, false, true, true, true, false, true, true, true, false, true, true, true, false,
        true, true,
    ];
    let integer: u64 = BoolArray::store(&bools).unwrap();
    println!("Integer: {}", integer); // 1096635
    println!("Length: {}", BoolArray::length(integer)); // 16
    println!("{:?}", BoolArray::retrieve(integer)); // [true, false, ..., true, true]
}
