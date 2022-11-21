use alloc_pool::{
    bytes::{
        Bytes,
        BytesMut,
    },
};

use crate::{
    integer,
    ReadFromBytes,
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
    NotEnoughBytes {
        required: usize,
        provided: usize,
    },
}

impl ReadFromBytes for Bytes {
    type Error = ReadBytesError;

    fn read_from_bytes(bytes: Bytes) -> Result<(Self, Bytes), Self::Error> {
        let (bytes_count, next_bytes) = u32::read_from_bytes(bytes)
            .map_err(ReadBytesError::BytesCount)?;
        let bytes_count: usize = bytes_count.try_into()
            .map_err(ReadBytesError::BytesCountConvert)?;
        if next_bytes.len() < bytes_count {
            return Err(ReadBytesError::NotEnoughBytes {
                required: bytes_count,
                provided: next_bytes.len(),
            });
        }
        let bytes_subrange = next_bytes.subrange(.. bytes_count);
        let next_bytes = next_bytes.subrange(bytes_count ..);
        Ok((bytes_subrange, next_bytes))
    }
}
