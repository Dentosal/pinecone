//! Pinecone, a minimalistic `no_std` + `alloc` serde format
//!
//! Works just like any other normal serde:
//!
//! ```rust
//! use pinecone::{from_bytes, to_slice, to_vec};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
//! struct Example {
//!     foo: String,
//!     bar: Option<u32>,
//!     zot: bool,
//! }
//!
//! let original = Example {
//!     foo: "Vec test".to_string(),
//!     bar: Some(0x1337),
//!     zot: true,
//! };
//!
//! let bytes: Vec<u8> = to_vec(&original).expect("Serialization failed");
//! assert_eq!(from_bytes(&bytes), Ok(original));
//!
//! let original = Example {
//!     foo: "Slice test".to_string(),
//!     bar: Some(0x1337),
//!     zot: true,
//! };
//!
//! let mut buffer = [0; 1024];
//! to_slice(&original, &mut buffer).expect("Serialization failed");
//! assert_eq!(from_bytes(&buffer), Ok(original));
//! ```

#![cfg_attr(not(feature = "use-std"), no_std)]
#![cfg_attr(not(feature = "use-std"), feature(alloc_prelude))]
// #![deny(missing_docs)]
#![allow(unused_imports)]

// #[cfg(all(test, not(feature = "use-std")))]
// compile_error!("Trying to run tests without std. Supply --features use-std to run.");

#[cfg(not(feature = "use-std"))]
extern crate alloc;

#[cfg(not(feature = "use-std"))]
mod prelude {
    pub use alloc::format;
    pub use alloc::prelude::v1::*;
    pub use hashbrown::HashMap;
}

#[cfg(feature = "use-std")]
mod prelude {
    pub use std::collections::HashMap;
}

mod de;
mod error;
mod ser;
mod varint;

pub use de::deserializer::Deserializer;
pub use de::{from_bytes, take_from_bytes};
pub use error::{Error, Result};
pub use ser::{serializer::Serializer, to_slice, to_vec};
