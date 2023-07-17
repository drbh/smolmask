use smolmask::BoolArray;

fn main() {
    // we can store up to 4 bools in a u16
    println!("Max u8 size: {}", BoolArray::<u8>::max_bools()); // 2
    println!("Max u16 size: {}", BoolArray::<u16>::max_bools()); // 4
    println!("Max u32 size: {}", BoolArray::<u32>::max_bools()); // 8
    println!("Max u64 size: {}", BoolArray::<u64>::max_bools()); // 16

    let bools = vec![true, false, true, true];
    let integer: u16 = BoolArray::store(&bools).unwrap();

    println!("Integer: {}", integer); // 75
    println!("Length: {}", BoolArray::length(integer)); // 4
    println!("{:?}", BoolArray::retrieve(integer)); // [true, false, true, true]
}
