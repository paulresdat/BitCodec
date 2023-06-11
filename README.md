# BitCodec
A general encoder/decoder in Rust

## Decoding to the Bits

This project is a work in progress.  The library is meant to be a general purpose bit encoder/decoder module where you can specify message specs using JSON that will detail the anatomy of a message down to the bit length, not byte length.  For example: 2 bits can represent 4 values, so you can encode a value of 0-3 in the message as 2 bits and then decode it back to a byte or even int32 in whatever application is consuming the message.

Data is often transmitted in this fashion when bandwidth is a concern.  A good example would be GNSS satellite messaging also known as RTCM messages.  Satellites package geographical calculations and satellite data into messages by concatenating those values as a stream of bits with endian padding to the next byte.  In turn they then concatenate those messages into a byte stream.  Each message has many values that need to be extracted at bit positions, not byte positions.

## More to Come

At the time of this writing, I have a successful decoder and am working on repeating fields and the encoder.