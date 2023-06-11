#[cfg(test)]
mod bitwise_ops_tests {
    use crate::encoder::bitwise_ops::{get_bits_left_pad, get_bits_byte_pad_left};

    #[test]
    fn can_read_bits_in_a_stream_of_bytes() {
        let bytes: [u8; 3] = [0b0011_1111, 0b0011_0011, 0b0000_0011];
    
        // works across bytes, that's good
        let new_b = get_bits_left_pad(&bytes, 0, 8);
        assert_eq!(new_b[0], 0b0011_1111);
        let new_b = get_bits_left_pad(&bytes, 1, 9);
        assert_eq!(new_b[0], 0b0111_1110);
        let new_b = get_bits_left_pad(&bytes, 2, 10);
        assert_eq!(new_b[0], 0b1111_1100);
    
        // // now does it work with more than one byte
        let mut new_b = get_bits_left_pad(&bytes, 0, 16);
        // note, this is without reversal so we need to reverse for the correct byte
        new_b.reverse();
        assert_eq!(new_b[0], 0b0011_1111);
        assert_eq!(new_b[1], 0b0011_0011);
    
        let mut new_b = get_bits_left_pad(&bytes, 1, 15);
        new_b.reverse();
        assert_eq!(new_b[0], 0b0001_1111);
        assert_eq!(new_b[1], 0b1001_1001);
    
        // SWEET!
        // this will be at the heart of the engine
    }
    
    #[test]
    fn can_pad_bits_to_expected_byte_length_for_conversion() {
        let bytes: [u8; 3] = [0b0011_1111, 0b0011_0011, 0b0000_0011];
        // 64 bits
        let new_b = get_bits_byte_pad_left(&bytes, 0, 8, 8);
        assert_eq!(8, new_b.len());
    }
}
