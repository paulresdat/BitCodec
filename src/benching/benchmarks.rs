// fn benchmark_get_bits() {
//     // setup benchmark
//     let v = vec![0,0,63];

//     let result = bench(|| {
//         // we are benchmarking how fast each iteration actually runs
//         get_bits(&v, 0, 1);
//     });

//     println!("Result | {}", result);
// }

// fn benchmark_get_bits2() {
//     // setup benchmark
//     let v = vec![0,0,63];

//     let result = bench(|| {
//         // we are benchmarking how fast each iteration actually runs
//         get_bits2(&v, 0, 1);
//     });

//     println!("Result | {}", result);
// }

// fn benchmark_get_bits3() {
//     // setup benchmark
//     let v = vec![0,0,63];

//     let result = bench(|| {
//         // we are benchmarking how fast each iteration actually runs
//         get_bits3(&v, 0, 1);
//     });

//     println!("Result | {}", result);
// }

// fn benchmark_get_byte() {
//     // setup benchmark
//     let b = 0b0011_0011;
//     let result = bench(|| {
//         get_byte(&b, 2, 3);
//     });

//     println!("Result | {}", result);
// }

// fn benchmark_get_bit() {
//     let b = 0b0011_0011;
//     let result = bench(|| {
//         get_bit(b, 2);
//     });
//     println!("Result | {}", result);
// }

// use crate::{encoder::bitwise_ops::get_bits_left_pad, bencher::bencher};

use crate::{benching::bencher::bencher, encoder::bitwise_ops::get_bits_left_pad};

// fastest times are around 300 nanoseconds
pub fn benchmark_bit_pad_left() {
    let b: [u8;1] = [0b0011_0011];
    let result = bencher(|| {
        get_bits_left_pad(&b, 0, 2);
    });
    println!("Result | {}", result);
}