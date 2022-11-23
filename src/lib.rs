use alloc_pool::{
    bytes::{
        Bytes,
        BytesMut,
        BytesPool,
    },
};

pub mod bytes;
pub mod integer;
pub mod combinators;

#[cfg(test)]
mod tests;

pub trait Target {
    fn extend_from_slice(&mut self, slice: &[u8]);
}

impl Target for BytesMut {
    fn extend_from_slice(&mut self, slice: &[u8]) {
        let vec: &mut Vec<_> = &mut *self;
        vec.extend_from_slice(slice);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct TargetCounter {
    bytes_written: usize,
}

impl TargetCounter {
    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}

impl Target for TargetCounter {
    fn extend_from_slice(&mut self, slice: &[u8]) {
        self.bytes_written += slice.len();
    }
}

pub trait WriteToBytesMut {
    fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target;
}

pub trait Source {
    fn slice(&self) -> &[u8];
    fn parent_bytes(&self) -> Option<&Bytes>;
    fn advance(&mut self, bytes_count: usize);
}

#[derive(Clone)]
pub struct SourceSlice<'a> {
    slice: &'a [u8],
}

impl<'a> From<&'a [u8]> for SourceSlice<'a> {
    fn from(slice: &'a [u8]) -> SourceSlice<'a> {
        SourceSlice { slice, }
    }
}

impl<'a> Source for SourceSlice<'a> {
    fn slice(&self) -> &[u8] {
        self.slice
    }

    fn parent_bytes(&self) -> Option<&Bytes> {
        None
    }

    fn advance(&mut self, bytes_count: usize) {
        self.slice = &self.slice[bytes_count ..];
    }
}

#[derive(Clone)]
pub struct SourceBytesRef<'a> {
    bytes: &'a Bytes,
    source_slice: SourceSlice<'a>,
}

impl<'a> From<&'a Bytes> for SourceBytesRef<'a> {
    fn from(bytes: &'a Bytes) -> SourceBytesRef<'a> {
        SourceBytesRef {
            bytes,
            source_slice: SourceSlice::from(&**bytes),
        }
    }
}

impl<'a> Source for SourceBytesRef<'a> {
    fn slice(&self) -> &[u8] {
        self.source_slice.slice()
    }

    fn parent_bytes(&self) -> Option<&Bytes> {
        Some(self.bytes)
    }

    fn advance(&mut self, bytes_count: usize) {
        self.source_slice.advance(bytes_count);
    }
}

#[derive(Clone)]
pub struct SourceBytesOwned {
    parent_bytes: Bytes,
    bytes: Bytes,
}

impl From<Bytes> for SourceBytesOwned {
    fn from(bytes: Bytes) -> SourceBytesOwned {
        SourceBytesOwned {
            parent_bytes: bytes.clone(),
            bytes,
        }
    }
}

impl Source for SourceBytesOwned {
    fn slice(&self) -> &[u8] {
        &self.bytes
    }

    fn parent_bytes(&self) -> Option<&Bytes> {
        Some(&self.parent_bytes)
    }

    fn advance(&mut self, bytes_count: usize) {
        self.bytes.focus_subrange(bytes_count ..);
    }
}

pub trait ReadFromSource: Sized {
    type Error;

    fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source;
}

pub fn write<T>(blocks_pool: &BytesPool, value: &T) -> Bytes where T: WriteToBytesMut {
    let mut bytes_mut = blocks_pool.lend();
    value.write_to_bytes_mut(&mut bytes_mut);
    bytes_mut.freeze()
}

impl<'a, U> WriteToBytesMut for &'a U where U: WriteToBytesMut {
    fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target {
        (*self).write_to_bytes_mut(target);
    }
}
