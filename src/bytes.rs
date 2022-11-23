use alloc_pool::{
    bytes::{
        Bytes,
        BytesMut,
    },
};

use crate::{
    integer,
    Source,
    ReadFromSource,
    WriteToBytesMut,
};

impl WriteToBytesMut for Bytes {
    fn write_to_bytes_mut(&self, bytes_mut: &mut BytesMut) {
        let bytes_len: u32 = self.len().try_into().unwrap();
        bytes_len.write_to_bytes_mut(bytes_mut);
        bytes_mut.extend_from_slice(self);
    }
}

#[derive(Debug)]
pub enum ReadBytesError {
    BytesCount(integer::ReadIntegerError),
    BytesCountConvert(std::num::TryFromIntError),
    NoBytesAvailable,
    NotEnoughBytes {
        required: usize,
        provided: usize,
    },
}

impl ReadFromSource for Bytes {
    type Error = ReadBytesError;

    fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source {
        let bytes_count = u32::read_from_source(source)
            .map_err(Self::Error::BytesCount)?;
        let bytes_count: usize = bytes_count.try_into()
            .map_err(Self::Error::BytesCountConvert)?;
        if source.slice().len() < bytes_count {
            return Err(Self::Error::NotEnoughBytes {
                required: bytes_count,
                provided: source.slice().len(),
            });
        }
        let parent_bytes = source.parent_bytes()
            .ok_or(Self::Error::NoBytesAvailable)?;
        let bytes_subrange = parent_bytes.clone_subslice(&source.slice()[.. bytes_count]);
        source.advance(bytes_count);
        Ok(bytes_subrange)
    }
}
