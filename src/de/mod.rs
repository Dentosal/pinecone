use serde::Deserialize;

pub(crate) mod deserializer;

use crate::error::Result;
use deserializer::Deserializer;

/// Deserialize a message of type `T` from a byte slice. The unused portion (if any)
/// of the byte slice is discarded
pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

/// Deserialize a message of type `T` from a byte slice. The unused portion (if any)
/// of the byte slice is returned for further usage
pub fn take_from_bytes<'a, T>(s: &'a [u8]) -> Result<(T, &'a [u8])>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok((t, deserializer.input))
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    #![allow(clippy::unreadable_literal)]

    use super::*;
    use crate::ser::to_vec;
    use core::fmt::Write;
    use core::ops::Deref;
    use serde::{Deserialize, Serialize};

    use crate::prelude::*;

    #[test]
    fn de_u8() {
        let output: Vec<u8> = to_vec(&0x05u8).unwrap();
        assert!([5] == output.deref());

        let out: u8 = from_bytes(output.deref()).unwrap();
        assert_eq!(out, 0x05);
    }

    #[test]
    fn de_u16() {
        let output: Vec<u8> = to_vec(&0xA5C7u16).unwrap();
        assert!([0xC7, 0xA5] == output.deref());

        let out: u16 = from_bytes(output.deref()).unwrap();
        assert_eq!(out, 0xA5C7);
    }

    #[test]
    fn de_u32() {
        let output: Vec<u8> = to_vec(&0xCDAB3412u32).unwrap();
        assert!([0x12, 0x34, 0xAB, 0xCD] == output.deref());

        let out: u32 = from_bytes(output.deref()).unwrap();
        assert_eq!(out, 0xCDAB3412u32);
    }

    #[test]
    fn de_u64() {
        let output: Vec<u8> = to_vec(&0x1234_5678_90AB_CDEFu64).unwrap();
        assert!([0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12] == output.deref());

        let out: u64 = from_bytes(output.deref()).unwrap();
        assert_eq!(out, 0x1234_5678_90AB_CDEFu64);
    }

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    struct BasicU8S {
        st: u16,
        ei: u8,
        sf: u64,
        tt: u32,
    }

    #[test]
    fn de_struct_unsigned() {
        let data = BasicU8S {
            st: 0xABCD,
            ei: 0xFE,
            sf: 0x1234_4321_ABCD_DCBA,
            tt: 0xACAC_ACAC,
        };

        let output: Vec<u8> = to_vec(&data).unwrap();

        assert!(
            [
                0xCD, 0xAB, 0xFE, 0xBA, 0xDC, 0xCD, 0xAB, 0x21, 0x43, 0x34, 0x12, 0xAC, 0xAC, 0xAC,
                0xAC
            ] == output.deref()
        );

        let out: BasicU8S = from_bytes(output.deref()).unwrap();
        assert_eq!(out, data);
    }

    #[test]
    fn de_byte_slice() {
        let input: &[u8] = &[1u8, 2, 3, 4, 5, 6, 7, 8];
        let output: Vec<u8> = to_vec(input).unwrap();
        assert_eq!(
            [0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
            output.deref()
        );

        let out: Vec<u8> = from_bytes(output.deref()).unwrap();
        assert_eq!(input, out.deref());

        let mut input: Vec<u8> = Vec::new();
        for i in 0..1024 {
            input.push((i & 0xFF) as u8);
        }
        let output: Vec<u8> = to_vec(input.deref()).unwrap();
        assert_eq!(&[0x80, 0x08], &output.deref()[..2]);

        assert_eq!(output.len(), 1026);
        for (i, val) in output.deref()[2..].iter().enumerate() {
            assert_eq!((i & 0xFF) as u8, *val);
        }

        let de: Vec<u8> = from_bytes(output.deref()).unwrap();
        assert_eq!(input.deref(), de.deref());
    }

    #[test]
    fn de_str() {
        let input: &str = "hello, pinecone!";
        let output: Vec<u8> = to_vec(input).unwrap();
        assert_eq!(0x10, output.deref()[0]);
        assert_eq!(input.as_bytes(), &output.deref()[1..]);

        let mut input: String = String::new();
        for _ in 0..256 {
            write!(&mut input, "abcd").unwrap();
        }
        let output: Vec<u8> = to_vec(input.deref()).unwrap();
        assert_eq!(&[0x80, 0x08], &output.deref()[..2]);

        assert_eq!(output.len(), 1026);
        for ch in output.deref()[2..].chunks(4) {
            assert_eq!("abcd", core::str::from_utf8(ch).unwrap());
        }

        let de: String = from_bytes(output.deref()).unwrap();
        assert_eq!(input.deref(), de.deref());
    }

    #[allow(dead_code)]
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    enum BasicEnum {
        Bib,
        Bim,
        Bap,
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct EnumStruct {
        eight: u8,
        sixt: u16,
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    enum DataEnum {
        Bib(u16),
        Bim(u64),
        Bap(u8),
        Kim(EnumStruct),
        Chi { a: u8, b: u32 },
        Sho(u16, u8),
    }

    #[test]
    fn enums() {
        let output: Vec<u8> = to_vec(&BasicEnum::Bim).unwrap();
        assert_eq!(&[0x01], output.deref());
        let out: BasicEnum = from_bytes(output.deref()).unwrap();
        assert_eq!(out, BasicEnum::Bim);

        let output: Vec<u8> = to_vec(&DataEnum::Bim(u64::max_value())).unwrap();
        assert_eq!(
            &[0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            output.deref()
        );

        let output: Vec<u8> = to_vec(&DataEnum::Bib(u16::max_value())).unwrap();
        assert_eq!(&[0x00, 0xFF, 0xFF], output.deref());
        let out: DataEnum = from_bytes(output.deref()).unwrap();
        assert_eq!(out, DataEnum::Bib(u16::max_value()));

        let output: Vec<u8> = to_vec(&DataEnum::Bap(u8::max_value())).unwrap();
        assert_eq!(&[0x02, 0xFF], output.deref());
        let out: DataEnum = from_bytes(output.deref()).unwrap();
        assert_eq!(out, DataEnum::Bap(u8::max_value()));

        let output: Vec<u8> = to_vec(&DataEnum::Kim(EnumStruct {
            eight: 0xF0,
            sixt: 0xACAC,
        }))
        .unwrap();
        assert_eq!(&[0x03, 0xF0, 0xAC, 0xAC,], output.deref());
        let out: DataEnum = from_bytes(output.deref()).unwrap();
        assert_eq!(
            out,
            DataEnum::Kim(EnumStruct {
                eight: 0xF0,
                sixt: 0xACAC
            })
        );

        let output: Vec<u8> = to_vec(&DataEnum::Chi {
            a: 0x0F,
            b: 0xC7C7C7C7,
        })
        .unwrap();
        assert_eq!(&[0x04, 0x0F, 0xC7, 0xC7, 0xC7, 0xC7], output.deref());
        let out: DataEnum = from_bytes(output.deref()).unwrap();
        assert_eq!(
            out,
            DataEnum::Chi {
                a: 0x0F,
                b: 0xC7C7C7C7
            }
        );

        let output: Vec<u8> = to_vec(&DataEnum::Sho(0x6969, 0x07)).unwrap();
        assert_eq!(&[0x05, 0x69, 0x69, 0x07], output.deref());
        let out: DataEnum = from_bytes(output.deref()).unwrap();
        assert_eq!(out, DataEnum::Sho(0x6969, 0x07));
    }

    #[test]
    fn tuples() {
        let output: Vec<u8> = to_vec(&(1u8, 10u32, "Hello!")).unwrap();
        assert_eq!(
            &[1u8, 0x0A, 0x00, 0x00, 0x00, 0x06, b'H', b'e', b'l', b'l', b'o', b'!'],
            output.deref()
        );
        let out: (u8, u32, &str) = from_bytes(output.deref()).unwrap();
        assert_eq!(out, (1u8, 10u32, "Hello!"));
    }

    #[test]
    fn bytes() {
        let x: &[u8; 32] = &[0u8; 32];
        let output: Vec<u8> = to_vec(x).unwrap();
        assert_eq!(output.len(), 32);
        let out: [u8; 32] = from_bytes(output.deref()).unwrap();
        assert_eq!(out, [0u8; 32]);
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    pub struct NewTypeStruct(u32);

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    pub struct PairStruct(u8, u16);

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    pub struct TupleStruct((u8, u16));

    #[test]
    fn structs() {
        let output: Vec<u8> = to_vec(&NewTypeStruct(5)).unwrap();
        assert_eq!(&[0x05, 0x00, 0x00, 0x00], output.deref());
        let out: NewTypeStruct = from_bytes(output.deref()).unwrap();
        assert_eq!(out, NewTypeStruct(5));

        let output: Vec<u8> = to_vec(&PairStruct(0xA0, 0x1234)).unwrap();
        assert_eq!(&[0xA0, 0x34, 0x12], output.deref());
        let out: PairStruct = from_bytes(output.deref()).unwrap();
        assert_eq!(out, PairStruct(0xA0, 0x1234));

        let output: Vec<u8> = to_vec(&TupleStruct((0xA0, 0x1234))).unwrap();
        assert_eq!(&[0xA0, 0x34, 0x12], output.deref());
        let out: TupleStruct = from_bytes(output.deref()).unwrap();
        assert_eq!(out, TupleStruct((0xA0, 0x1234)));
    }

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct RefStruct<'a> {
        bytes: &'a [u8],
        str_s: &'a str,
    }

    #[test]
    fn ref_struct() {
        let message = "hElLo";
        let bytes = [0x01, 0x10, 0x02, 0x20];
        let output: Vec<u8> = to_vec(&RefStruct {
            bytes: &bytes,
            str_s: message,
        })
        .unwrap();

        assert_eq!(
            &[0x04, 0x01, 0x10, 0x02, 0x20, 0x05, b'h', b'E', b'l', b'L', b'o',],
            output.deref()
        );

        let out: RefStruct = from_bytes(output.deref()).unwrap();
        assert_eq!(
            out,
            RefStruct {
                bytes: &bytes,
                str_s: message,
            }
        );
    }

    #[test]
    fn unit() {
        #![allow(clippy::let_unit_value)]
        let output: Vec<u8> = to_vec(&()).unwrap();
        assert_eq!(output.len(), 0);
        let _: () = from_bytes(output.deref()).unwrap();
    }

    #[test]
    fn vec() {
        let mut input: Vec<u8> = Vec::new();
        input.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        let output: Vec<u8> = to_vec(&input).unwrap();
        assert_eq!(&[0x04, 0x01, 0x02, 0x03, 0x04], output.deref());
        let out: Vec<u8> = from_bytes(output.deref()).unwrap();
        assert_eq!(out, input);

        let mut input: String = String::new();
        write!(&mut input, "helLO!").unwrap();
        let output: Vec<u8> = to_vec(&input).unwrap();
        assert_eq!(&[0x06, b'h', b'e', b'l', b'L', b'O', b'!'], output.deref());
        let out: String = from_bytes(output.deref()).unwrap();
        assert_eq!(input, out);
    }

    #[test]
    fn hashmap() {
        let result: HashMap<u8, u8> = from_bytes(&[0]).unwrap();
        assert!(result.is_empty());

        let mut hm = HashMap::new();
        hm.insert(1, 2);
        hm.insert(3, 4);
        hm.insert(5, 6);

        let result: HashMap<u8, u8> = from_bytes(&[3, 1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(result, hm);
    }
}
