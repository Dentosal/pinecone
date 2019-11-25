use core::ops::Index;
use core::ops::IndexMut;

use crate::prelude::*;

/// Generic serialization target
pub trait SerOutput {
    /// Result of the serialization
    type Output;

    /// Can be implemented when there is a more efficient way of processing
    /// multiple bytes at once, such as copying a slice to the output, rather
    /// than iterating over one byte at a time.
    fn try_extend(&mut self, data: &[u8]) -> core::result::Result<(), ()> {
        data.iter()
            .try_for_each(|d| self.try_push(*d))
            .map_err(|_| ())
    }

    /// Pushes a single byte to be stored
    fn try_push(&mut self, data: u8) -> core::result::Result<(), ()>;

    /// Finalizes the storage operation, and resolved into associated type.
    fn release(self) -> core::result::Result<Self::Output, ()>;
}

/// Stores the serialized bytes into a plain `[u8]` slice.
/// Resolves into a sub-slice of the original slice buffer.
pub struct SliceOutput<'a> {
    buf: &'a mut [u8],
    idx: usize,
}

impl<'a> SliceOutput<'a> {
    /// Create from a given backing buffer
    pub fn new(buf: &'a mut [u8]) -> Self {
        SliceOutput { buf, idx: 0 }
    }
}

impl<'a> SerOutput for SliceOutput<'a> {
    type Output = &'a mut [u8];

    fn try_extend(&mut self, data: &[u8]) -> core::result::Result<(), ()> {
        let len = data.len();

        if (len + self.idx) > self.buf.len() {
            return Err(());
        }

        self.buf[self.idx..self.idx + len].copy_from_slice(data);

        self.idx += len;

        Ok(())
    }

    fn try_push(&mut self, data: u8) -> core::result::Result<(), ()> {
        if self.idx >= self.buf.len() {
            return Err(());
        }

        self.buf[self.idx] = data;
        self.idx += 1;

        Ok(())
    }

    fn release(self) -> core::result::Result<Self::Output, ()> {
        let (used, _unused) = self.buf.split_at_mut(self.idx);
        Ok(used)
    }
}

impl<'a> Index<usize> for SliceOutput<'a> {
    type Output = u8;

    fn index(&self, idx: usize) -> &u8 {
        &self.buf[idx]
    }
}

impl<'a> IndexMut<usize> for SliceOutput<'a> {
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        &mut self.buf[idx]
    }
}

/// Wrapper type around a `Vec`.
pub struct VecOutput(pub Vec<u8>);

impl VecOutput {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl SerOutput for VecOutput {
    type Output = Vec<u8>;

    #[inline(always)]
    fn try_extend(&mut self, data: &[u8]) -> core::result::Result<(), ()> {
        self.0.extend_from_slice(data);
        Ok(())
    }

    #[inline(always)]
    fn try_push(&mut self, data: u8) -> core::result::Result<(), ()> {
        self.0.push(data);
        Ok(())
    }

    fn release(self) -> core::result::Result<Self::Output, ()> {
        Ok(self.0)
    }
}

impl Index<usize> for VecOutput {
    type Output = u8;

    fn index(&self, idx: usize) -> &u8 {
        &self.0[idx]
    }
}

impl IndexMut<usize> for VecOutput {
    fn index_mut(&mut self, idx: usize) -> &mut u8 {
        &mut self.0[idx]
    }
}
