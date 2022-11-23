use std::{
    mem,
};

use crate::{
    Source,
    Target,
    ReadFromSource,
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
            fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target {
                target.extend_from_slice(&self.to_be_bytes());
            }
        }

        impl ReadFromSource for $T {
            type Error = ReadIntegerError;

            fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source {
                let required = mem::size_of::<Self>();
                let slice = source.slice();
                let value = slice.get(.. required)
                    .and_then(|octets_slice| octets_slice.try_into().ok())
                    .map(|octets| Self::from_be_bytes(octets))
                    .ok_or(Self::Error::InvalidBytesCount {
                        provided: slice.len(),
                        required,
                    })?;
                source.advance(required);
                Ok(value)
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
