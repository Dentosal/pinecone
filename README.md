# Pinecone - Yet another binary format for Serde

Pinecone is a minimalistic `no_std` + `alloc` fork of [Postcard](https://github.com/jamesmunns/postcard).

* [Documentation](https://docs.rs/pinecone/)
* [Crates.io](https://crates.io/crates/pinecone)

## Features

Pinecone always assumes that deserialization target is correct.
It is fully possible to deserialize into an incorrect type.
However, this provides requires less space and is faster to decode speed.

## Usage

Works just like any other normal serde:

```rust
use pinecone::{from_bytes, to_slice, to_vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Example {
    foo: String,
    bar: Option<u32>,
    zot: bool,
}

fn main() {
    let original = Example {
        foo: "Vec test".to_string(),
        bar: Some(0x1337),
        zot: true,
    };

    let bytes: Vec<u8> = to_vec(&original).expect("Serialization failed");
    assert_eq!(from_bytes(&bytes), Ok(original));

    let original = Example {
        foo: "Slice test".to_string(),
        bar: Some(0x1337),
        zot: true,
    };

    let mut buffer = [0; 1024];
    to_slice(&original, &mut buffer).expect("Serialization failed");
    assert_eq!(from_bytes(&buffer), Ok(original));
}
```

## Variable Length Data

Variable length data (such as slices) are prefixed by their length.

Length is encoded as a [Varint]. This is done for two reasons: to minimize wasted bytes
on the wire when sending slices with items less than 127 items (typical for embedded),
and to reduce compatibility issues between 32-bit and 64-bit targets due to differing sizes
of `usize`.

Similarly, `enum` descriminants are encoded as varints, meaning that any enum with less than
127 variants will encode its discriminant as a single byte (rather than a `u32`).

Varints in `pinecone` have a maximum value of the usize for that platform. In practice, this
means that 64-bit targets should not send messages with slices containing `(1 << 32) - 1` items
to 32-bit targets, which is uncommon in practice. Enum discriminants already have a fixed
maximum value of `(1 << 32) - 1` as currently defined in Rust. Varints larger than the current platform's
`usize` will cause the deserialization process to return an `Err`.

[Varint]: https://developers.google.com/protocol-buffers/docs/encoding

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
