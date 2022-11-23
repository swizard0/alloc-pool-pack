use either::{
    Either,
};

use crate::{
    integer,
    Source,
    Target,
    ReadFromSource,
    WriteToBytesMut,
};

// Unit

impl WriteToBytesMut for () {
    fn write_to_bytes_mut<T>(&self, _target: &mut T) where T: Target {
    }
}

#[derive(Debug)]
pub enum ReadUnitError {
}

impl ReadFromSource for () {
    type Error = ReadUnitError;

    fn read_from_source<S>(_source: &mut S) -> Result<Self, Self::Error> where S: Source {
        Ok(())
    }
}

// Either

const TAG_LEFT: u8 = 1;
const TAG_RIGHT: u8 = 2;

impl<L, R> WriteToBytesMut for Either<L, R> where L: WriteToBytesMut, R: WriteToBytesMut {
    fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target {
        match self {
            Either::Left(left) => {
                TAG_LEFT.write_to_bytes_mut(target);
                left.write_to_bytes_mut(target);
            },
            Either::Right(right) => {
                TAG_RIGHT.write_to_bytes_mut(target);
                right.write_to_bytes_mut(target);
            },
        }
    }
}

#[derive(Debug)]
pub enum ReadEitherError<L, R> where L: ReadFromSource, R: ReadFromSource {
    Tag(integer::ReadIntegerError),
    InvalidTag(u8),
    Left(L::Error),
    Right(R::Error),
}

impl<L, R> ReadFromSource for Either<L, R> where L: ReadFromSource, R: ReadFromSource {
    type Error = ReadEitherError<L, R>;

    fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source {
        let tag = u8::read_from_source(source)
            .map_err(Self::Error::Tag)?;
        match tag {
            TAG_LEFT => {
                let left = L::read_from_source(source)
                    .map_err(Self::Error::Left)?;
                Ok(Either::Left(left))
            },
            TAG_RIGHT => {
                let right = R::read_from_source(source)
                    .map_err(Self::Error::Right)?;
                Ok(Either::Right(right))
            },
            _ =>
                Err(Self::Error::InvalidTag(tag)),
        }
    }
}

// Option

impl<U> WriteToBytesMut for Option<U> where U: WriteToBytesMut {
    fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target {
        let either = match self {
            None =>
                Either::Left(()),
            Some(value) =>
                Either::Right(value),
        };
        either.write_to_bytes_mut(target);
    }
}

impl<U> ReadFromSource for Option<U> where U: ReadFromSource {
    type Error = ReadEitherError<(), U>;

    fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source {
        let either = Either::read_from_source(source)?;
        let option = match either {
            Either::Left(()) =>
                None,
            Either::Right(value) =>
                Some(value),
        };
        Ok(option)
    }
}

// Result

impl<U, E> WriteToBytesMut for Result<U, E> where U: WriteToBytesMut, E: WriteToBytesMut {
    fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target {
        let either = match self {
            Err(error) =>
                Either::Left(error),
            Ok(value) =>
                Either::Right(value),
        };
        either.write_to_bytes_mut(target);
    }
}

impl<U, E> ReadFromSource for Result<U, E> where U: ReadFromSource, E: ReadFromSource {
    type Error = ReadEitherError<E, U>;

    fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source {
        let either = Either::read_from_source(source)?;
        let result = match either {
            Either::Left(error) =>
                Err(error),
            Either::Right(value) =>
                Ok(value),
        };
        Ok(result)
    }
}

// tuple2

impl<A, B> WriteToBytesMut for (A, B) where A: WriteToBytesMut, B: WriteToBytesMut {
    fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target {
        self.0.write_to_bytes_mut(target);
        self.1.write_to_bytes_mut(target);
    }
}

#[derive(Debug)]
pub enum ReadTuple2Error<A, B> where A: ReadFromSource, B: ReadFromSource {
    A(A::Error),
    B(B::Error),
}

impl<A, B> ReadFromSource for (A, B) where A: ReadFromSource, B: ReadFromSource {
    type Error = ReadTuple2Error<A, B>;

    fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source {
        let a = A::read_from_source(source)
            .map_err(ReadTuple2Error::A)?;
        let b = B::read_from_source(source)
            .map_err(ReadTuple2Error::B)?;
        Ok((a, b))
    }
}

// tuple3

impl<A, B, C> WriteToBytesMut for (A, B, C) where A: WriteToBytesMut, B: WriteToBytesMut, C: WriteToBytesMut {
    fn write_to_bytes_mut<T>(&self, target: &mut T) where T: Target {
        self.0.write_to_bytes_mut(target);
        self.1.write_to_bytes_mut(target);
        self.2.write_to_bytes_mut(target);
    }
}

#[derive(Debug)]
pub enum ReadTuple3Error<A, B, C> where A: ReadFromSource, B: ReadFromSource, C: ReadFromSource {
    A(A::Error),
    B(B::Error),
    C(C::Error),
}

impl<A, B, C> ReadFromSource for (A, B, C) where A: ReadFromSource, B: ReadFromSource, C: ReadFromSource {
    type Error = ReadTuple3Error<A, B, C>;

    fn read_from_source<S>(source: &mut S) -> Result<Self, Self::Error> where S: Source {
        let a = A::read_from_source(source)
            .map_err(ReadTuple3Error::A)?;
        let b = B::read_from_source(source)
            .map_err(ReadTuple3Error::B)?;
        let c = C::read_from_source(source)
            .map_err(ReadTuple3Error::C)?;
        Ok((a, b, c))
    }
}
