// use chrono::{DateTime, TimeZone, Utc};

// use std::io::{stdout, Write};

/// This function gets the bit value as a boolean from the byte
pub fn get_bit(byte: u8, pos: u8) -> bool {
    ((byte >> pos) & 1) == 1
}

pub struct IntConverter {

}

impl IntConverter {
    pub fn to_uint128(bytes: Vec<u8>) -> u128 {
        return u128::from_be_bytes(bytes[bytes.len()-16..].try_into().unwrap());
    }

    pub fn to_uint64(bytes: Vec<u8>) -> u64 {
        return u64::from_be_bytes(bytes[bytes.len()-8..].try_into().unwrap());
    }

    pub fn to_uint32(bytes: Vec<u8>) -> u32 {
        return u32::from_be_bytes(bytes[bytes.len()-4..].try_into().unwrap());
    }

    pub fn to_uint16(bytes: Vec<u8>) -> u16 {
        return u16::from_be_bytes(bytes[bytes.len()-2..].try_into().unwrap());
    }

    pub fn to_uint8(bytes: Vec<u8>) -> u8 {
        return u8::from_be_bytes(bytes[bytes.len()-1..].try_into().unwrap());
    }

    pub fn to_int128(bytes: Vec<u8>) -> i128 {
        // let bytes2 = pad_vec(bytes, 8);
        return i128::from_be_bytes(bytes[bytes.len()-16..].try_into().unwrap());
    }

    pub fn to_int64(bytes: Vec<u8>) -> i64 {
        // let bytes2 = pad_vec(bytes, 8);
        return i64::from_be_bytes(bytes[bytes.len()-8..].try_into().unwrap());
    }

    pub fn to_int32(bytes: Vec<u8>) -> i32 {
        return i32::from_be_bytes(bytes[bytes.len()-4..].try_into().unwrap());
    }

    pub fn to_int16(bytes: Vec<u8>) -> i16 {
        return i16::from_be_bytes(bytes[bytes.len()-2..].try_into().unwrap());
    }

    pub fn to_int8(bytes: Vec<u8>) -> i8 {
        return i8::from_be_bytes(bytes[bytes.len()-1..].try_into().unwrap());
    }

    pub fn to_f64(bytes: Vec<u8>) -> f64 {
        // let bytes2 = pad_vec(bytes, 8);
        return f64::from_be_bytes(bytes[bytes.len()-8..].try_into().unwrap());
    }

    pub fn to_f32(bytes: Vec<u8>) -> f32 {
        return f32::from_be_bytes(bytes[bytes.len()-4..].try_into().unwrap());
    }
}


// fn pad_vec(bytes: Vec<u8>, of_arbitrary_size: usize) -> Vec<u8> {
//     let len = bytes.len();
//     let mut bytes2 = bytes;
//     prepend(&mut bytes2, 0, if len < of_arbitrary_size {(of_arbitrary_size - len) as usize } else { 0 as usize });
//     return bytes2;
// }

/// This flips a bit in a byte at the position specified to the boolean value supplied
fn flip_bit(byte: u8, pos: u8, to_value: bool) -> u8 {
    let mask = 1 << pos;
    let b = if to_value { 1 } else { 0 };
    let byte = (byte & !mask) | ((b << pos) & mask);
    return byte;
}

// pub fn get_bytes(bits: &Vec<bool>) -> Vec<u8> {
//     // off amounts are always padded to the right for now (little endian)
//     let bit_length = bits.len();
//     let number_of_bits = (8 - (bit_length % 8)) + (bit_length);
//     let mut bytes: Vec<u8> = vec![0;number_of_bits/8];
//     let mut bits2 = bits.clone();
//     prepend(&mut bits2, false, number_of_bits - bit_length);
//     for i in 0 .. bits2.len() {
//         let current_byte = i / 8;
//         // start at the end and go up
//         let c = 8 - (i % 8) - 1;
//         bytes[current_byte] = flip_bit(bytes[current_byte], c as u8, bits2[i])
//     }
//     return bytes;
// }

// fn prepend<T: Clone>(v: &mut Vec<T>, x: T, n: usize) {
//     v.resize(v.len() + n, x);
//     v.rotate_right(n);
// }

/// This function ensures bits that are fetched are byte padded, it is the wrapper to the
/// get_bits heart of the decoder
pub fn get_bits_byte_pad_left(bytes: &[u8], start: usize, end: usize, pad_bytes: usize) -> Vec<u8> {
    let mut return_bytes = get_bits_left_pad(bytes, start, end);
    if pad_bytes > return_bytes.len() {
        let len = return_bytes.len();
        for _ in len .. pad_bytes {
            return_bytes.push(0u8);
        }
    }
    return_bytes.reverse();

    return_bytes
}

/// The main heart of the bit encoding algorithm.  This method takes in a byte array (maybe vector in the future)
/// and returns a new byte that strips out the 
/// Input
///   -> an array of bytes
///   -> starting bit position (starting at 0 to the left)
///   -> ending bit position (ending at end of the array to the right)
/// Output
///   -> either a left or right padded byte stream
/// Algorithm
///   -> if left pad
///     -> start at the ending bit and go down the array filling in the bytes to the start
///     -> this will automatically pad to the left
///   -> if right pad
///     -> panic for now
pub fn get_bits_left_pad(bytes: &[u8], start: usize, end: usize) -> Vec<u8> {
    let mut return_bytes: Vec<u8> = Vec::new();
    if start >= end {
        return return_bytes;
    }

    let mut c = 0;
    return_bytes.push(0u8);
    let mut last = return_bytes.len()-1;
    // we are marching down the bit stream from the right to the left
    // so that when we end, we end on a byte that is padded to the left
    for i in (start..end+1).rev() {
        if c == 8 {
            return_bytes.push(0u8);
            last = return_bytes.len()-1;
            c = 0;
        }
        let current_byte_pos = i / 8;
        let pos = i - (current_byte_pos * 8);
        let current_byte = bytes[current_byte_pos];
        // 7-pos flips the bit to start at the right instead of the left
        let current_bit = get_bit(current_byte, (7-pos) as u8);
        // println!("cbp {}, i {}, pos {}, bit {}", current_byte_pos, i, 7 - pos, current_bit);
        return_bytes[last] = flip_bit(return_bytes[last], c, current_bit);
        c += 1;
    }

    // we don't want to reverse the byte order for little endian ness
    // instead we just want to return the data
    return return_bytes;
}



// These 2 functions are the right approach, however there is a bug that requires a refactor
// The bug is that if there is a sequence of bits that spans over a couple bytes, it's not able
// to keep track of the bit position of the new byte independently of the sequence of bits from
// the original byte array.

// this is faster!  around 40-228 nanoseconds per execution
// pub fn get_byte(byte: &u8, start: usize, end: usize) -> u8 {
//     // println!("get_byte::start, end : {}, {}", start, end);
//     let m_start = 7 - start;
//     let m_end = 7 - end;
//     let mut byte_new = 0u8;
//     let beg = m_start as u8;
//     for i in (m_end .. m_start+1).rev() {
//         let b = get_bit(*byte, i as u8);
//         let p = beg - i as u8;
//         // println!("Idx {} as Bit {} to position {}", i, b, p);
//         byte_new = flip_bit(byte_new, p, b);
//     }
//     byte_new
// }

// definitely faster, around 300 nanoseconds in compared to older get_bits
// pub fn get_bits2(bytes: &[u8], start: usize, end: usize) -> Vec<u8> {
//     let start_byte = start / 8;
//     let end_byte = end / 8;
//     let actual_start = start - (start_byte * 8);
//     let actual_end = end - (end_byte * 8);
//     // println!("Start/End Bytes: {}, {} | {}, {}", start_byte, end_byte, actual_start, actual_end);
//     let mut new_bytes: Vec<u8> = Vec::new();
//     for i in start_byte .. end_byte+1 {
//         let bit_start = if i == start_byte { actual_start } else { 0 };
//         let bit_end = if i == end_byte { actual_end } else { 7 };
//         // println!("{}, bs {}, be {}", i, bit_start, bit_end);
//         let new_byte = get_byte(&bytes[i], bit_start, bit_end);
//         new_bytes.push(new_byte);
//     }
//     new_bytes
// }

// this is slow! around 1500 nanoseconds per execution, BUT it does exactly
// what I want it to do although half the job.
// pub fn get_bits(bytes: &[u8], start: usize, end: usize) -> Vec<bool> {
//     // println!("length: {}", end - start);
//     let mut a: Vec<Vec<bool>> = Vec::new();
//     let beginning_byte = start / 8;
//     let ending_byte = end / 8;
//     for _ in beginning_byte .. ending_byte+1 {
//         a.push(Vec::new());
//     }
//     let start_byte = start / 8;
//     for i in start .. end+1 {
//         let current_byte = i / 8;
//         let byte_index = current_byte - start_byte;
//         let c = i % 8;
//         if c == 0 && current_byte != 0 {
//             a[byte_index].reverse();
//         }
//         let d = get_bit(bytes[current_byte], c as u8);
//         a[byte_index].push(d)
//     }
//     let l = a.len()-1;
//     a[l].reverse();
//     let mut b: Vec<bool> = Vec::new();
//     for i in a {
//         for j in i {
//             b.push(j);
//         }
//     }
//     b
// }
