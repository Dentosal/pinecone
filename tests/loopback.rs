#![allow(clippy::unreadable_literal)]

use core::fmt::Debug;
use core::fmt::Write;
use core::ops::Deref;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use pinecone::{from_bytes, to_vec};

use hashbrown::HashMap;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct BasicU8S {
    st: u16,
    ei: u8,
    sf: u64,
    tt: u32,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct NewTypeStruct(u32);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct PairStruct(u8, u16);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct TupleStruct((u8, u16));

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct GenericVector<T>(Vec<T>);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct IntMapping(HashMap<u8, u8>);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct StringMapping(HashMap<String, String>);

#[test]
fn loopback() {
    // Basic types
    test_one((), &[]);
    test_one(false, &[0x00]);
    test_one(true, &[0x01]);
    test_one(5u8, &[0x05]);
    test_one(0xA5C7u16, &[0xC7, 0xA5]);
    test_one(0xCDAB3412u32, &[0x12, 0x34, 0xAB, 0xCD]);
    test_one(
        0x1234_5678_90AB_CDEFu64,
        &[0xEF, 0xCD, 0xAB, 0x90, 0x78, 0x56, 0x34, 0x12],
    );

    // Structs
    test_one(
        BasicU8S {
            st: 0xABCD,
            ei: 0xFE,
            sf: 0x1234_4321_ABCD_DCBA,
            tt: 0xACAC_ACAC,
        },
        &[
            0xCD, 0xAB, 0xFE, 0xBA, 0xDC, 0xCD, 0xAB, 0x21, 0x43, 0x34, 0x12, 0xAC, 0xAC, 0xAC,
            0xAC,
        ],
    );

    // Enums
    test_one(BasicEnum::Bim, &[0x01]);
    test_one(
        DataEnum::Bim(u64::max_value()),
        &[0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
    );
    test_one(DataEnum::Bib(u16::max_value()), &[0x00, 0xFF, 0xFF]);
    test_one(DataEnum::Bap(u8::max_value()), &[0x02, 0xFF]);
    test_one(
        DataEnum::Kim(EnumStruct {
            eight: 0xF0,
            sixt: 0xACAC,
        }),
        &[0x03, 0xF0, 0xAC, 0xAC],
    );
    test_one(
        DataEnum::Chi {
            a: 0x0F,
            b: 0xC7C7C7C7,
        },
        &[0x04, 0x0F, 0xC7, 0xC7, 0xC7, 0xC7],
    );
    test_one(DataEnum::Sho(0x6969, 0x07), &[0x05, 0x69, 0x69, 0x07]);

    // Tuples
    test_one((0x12u8, 0xC7A5u16), &[0x12, 0xA5, 0xC7]);

    // Structs
    test_one(NewTypeStruct(5), &[0x05, 0x00, 0x00, 0x00]);
    test_one(PairStruct(0xA0, 0x1234), &[0xA0, 0x34, 0x12]);
    test_one(TupleStruct((0xA0, 0x1234)), &[0xA0, 0x34, 0x12]);

    let mut input: Vec<u8> = Vec::new();
    input.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]);
    test_one(input, &[0x04, 0x01, 0x02, 0x03, 0x04]);

    let mut input: String = String::new();
    write!(&mut input, "helLO!").unwrap();
    test_one(input, &[0x06, b'h', b'e', b'l', b'L', b'O', b'!']);

    test_one(
        GenericVector(vec![String::new(), "a".to_string(), "bc".to_string()]),
        &[3, 0, 1, b'a', 2, b'b', b'c'],
    );

    // Chars
    test_opaque('a');
    test_opaque('ä');
    test_opaque('ह');
    test_opaque('€');
    test_opaque('한');
    test_opaque('𐍈');

    // Data containers
    test_opaque(IntMapping(HashMap::new()));
    test_opaque(StringMapping(HashMap::new()));
    // test_opaque(IntMapping(hashmap! {1 => 2, 3 => 4}));
    // test_opaque(StringMapping(
    //     hashmap! {"a".to_string() => "b".to_string(), "cd".to_string() => "ef".to_string()},
    // ));
}

fn test_one<T>(data: T, ser_rep: &[u8])
where
    T: Serialize + DeserializeOwned + Eq + PartialEq + Debug,
{
    let serialized: Vec<u8> = to_vec(&data).unwrap();
    assert_eq!(serialized.len(), ser_rep.len());
    let mut x: Vec<u8> = vec![];
    x.extend(serialized.deref().iter().cloned());
    assert_eq!(x, ser_rep);
    {
        let deserialized: T = from_bytes(&x).unwrap();
        assert_eq!(data, deserialized);
    }
}

fn test_opaque<T>(data: T)
where
    T: Serialize + DeserializeOwned + Eq + PartialEq + Debug,
{
    let serialized: Vec<u8> = to_vec(&data).unwrap();
    let mut x: Vec<u8> = vec![];
    x.extend(serialized.deref().iter().cloned());
    println!("SER {:?}", x);
    {
        let deserialized: T = from_bytes(&x).unwrap();
        assert_eq!(data, deserialized);
    }
}
