use std::{
    mem,
};

use alloc_pool::{
    bytes::{
        Bytes,
        BytesMut,
    },
};

use crate::{
    ReadFromBytes,
    WriteToBytesMut,
};

#[derive(Debug)]
pub enum ReadIntegerError {
    InvalidBytesCount {
        provided: usize,
        required: usize,
    },
}

macro_rules! traits_impl {
    ($T:ty) => {
        impl WriteToBytesMut for $T {
            fn write_to_bytes_mut(&self, bytes_mut: &mut BytesMut) {
                bytes_mut.extend_from_slice(&self.to_be_bytes());
            }
        }

        impl ReadFromBytes for $T {
            type Error = ReadIntegerError;

            fn read_from_bytes(bytes: Bytes) -> Result<(Self, Bytes), Self::Error> {
                let required = mem::size_of::<Self>();
                let value = bytes.get(.. required)
                    .and_then(|octets_slice| octets_slice.try_into().ok())
                    .map(|octets| Self::from_be_bytes(octets))
                    .ok_or(ReadIntegerError::InvalidBytesCount {
                        provided: bytes.len(),
                        required,
                    })?;
                let next_bytes = bytes.subrange(required ..);
                Ok((value, next_bytes))
            }
        }
    }
}

traits_impl!(u8);
traits_impl!(u16);
traits_impl!(u32);
traits_impl!(u64);
traits_impl!(u128);
traits_impl!(i8);
traits_impl!(i16);
traits_impl!(i32);
traits_impl!(i64);
traits_impl!(i128);
