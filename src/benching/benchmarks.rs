use crate::{benching::bencher::bencher, encoder::bitwise_ops::get_bits_left_pad};

// fastest times are around 300 nanoseconds
#[allow(unused)]
pub fn benchmark_bit_pad_left() {
    let b: [u8;1] = [0b0011_0011];
    let result = bencher(|| {
        get_bits_left_pad(&b, 0, 2);
    });
    println!("Result | {}", result);
}