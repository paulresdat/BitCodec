# BitCodec
A general encoder/decoder in Rust

This is still very much a work in progres. For the time being this is here so that I can have a git repository for the work.

This library is a general purpose low level implementation of compacted messages where values need to be represented using an arbitrary number of bits as defined in a spec.  Data is often transmitted in this fashion when bandwidth is a concern.  A good example would GNSS satellite messaging (also known as RTCM messages).  Satellites package values into a message and then concatenate those messages into a byte stream.  Each message has many values that need to be extracted at bit positions, not byte positions.

Once I've got this working, I'll explain more.