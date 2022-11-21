use either::{
    Either,
};

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

// Unit

impl WriteToBytesMut for () {
    fn write_to_bytes_mut(&self, _bytes_mut: &mut BytesMut) {
    }
}

#[derive(Debug)]
pub enum ReadUnitError {
}

impl ReadFromBytes for () {
    type Error = ReadUnitError;

    fn read_from_bytes(bytes: Bytes) -> Result<(Self, Bytes), Self::Error> {
        Ok(((), bytes))
    }
}

// Either

const TAG_LEFT: u8 = 1;
const TAG_RIGHT: u8 = 2;

impl<L, R> WriteToBytesMut for Either<L, R> where L: WriteToBytesMut, R: WriteToBytesMut {
    fn write_to_bytes_mut(&self, bytes_mut: &mut BytesMut) {
        match self {
            Either::Left(left) => {
                TAG_LEFT.write_to_bytes_mut(bytes_mut);
                left.write_to_bytes_mut(bytes_mut);
            },
            Either::Right(right) => {
                TAG_RIGHT.write_to_bytes_mut(bytes_mut);
                right.write_to_bytes_mut(bytes_mut);
            },
        }
    }
}

#[derive(Debug)]
pub enum ReadEitherError<L, R> where L: ReadFromBytes, R: ReadFromBytes {
    Tag(integer::ReadIntegerError),
    InvalidTag(u8),
    Left(L::Error),
    Right(R::Error),
}

impl<L, R> ReadFromBytes for Either<L, R> where L: ReadFromBytes, R: ReadFromBytes {
    type Error = ReadEitherError<L, R>;

    fn read_from_bytes(bytes: Bytes) -> Result<(Self, Bytes), Self::Error> {
        let (tag, bytes) = u8::read_from_bytes(bytes)
            .map_err(Self::Error::Tag)?;
        match tag {
            TAG_LEFT => {
                let (left, bytes) = L::read_from_bytes(bytes)
                    .map_err(Self::Error::Left)?;
                Ok((Either::Left(left), bytes))
            },
            TAG_RIGHT => {
                let (right, bytes) = R::read_from_bytes(bytes)
                    .map_err(Self::Error::Right)?;
                Ok((Either::Right(right), bytes))
            },
            _ =>
                Err(Self::Error::InvalidTag(tag)),
        }
    }
}

// Option

impl<T> WriteToBytesMut for Option<T> where T: WriteToBytesMut {
    fn write_to_bytes_mut(&self, bytes_mut: &mut BytesMut) {
        let either = match self {
            None =>
                Either::Left(()),
            Some(value) =>
                Either::Right(value),
        };
        either.write_to_bytes_mut(bytes_mut);
    }
}

impl<T> ReadFromBytes for Option<T> where T: ReadFromBytes {
    type Error = ReadEitherError<(), T>;

    fn read_from_bytes(bytes: Bytes) -> Result<(Self, Bytes), Self::Error> {
        let (either, bytes) = Either::read_from_bytes(bytes)?;
        let option = match either {
            Either::Left(()) =>
                None,
            Either::Right(value) =>
                Some(value),
        };
        Ok((option, bytes))
    }
}

// Result

impl<T, E> WriteToBytesMut for Result<T, E> where T: WriteToBytesMut, E: WriteToBytesMut {
    fn write_to_bytes_mut(&self, bytes_mut: &mut BytesMut) {
        let either = match self {
            Err(error) =>
                Either::Left(error),
            Ok(value) =>
                Either::Right(value),
        };
        either.write_to_bytes_mut(bytes_mut);
    }
}

impl<T, E> ReadFromBytes for Result<T, E> where T: ReadFromBytes, E: ReadFromBytes {
    type Error = ReadEitherError<E, T>;

    fn read_from_bytes(bytes: Bytes) -> Result<(Self, Bytes), Self::Error> {
        let (either, bytes) = Either::read_from_bytes(bytes)?;
        let result = match either {
            Either::Left(error) =>
                Err(error),
            Either::Right(value) =>
                Ok(value),
        };
        Ok((result, bytes))
    }
}
