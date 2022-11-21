use alloc_pool::{
    bytes::{
        Bytes,
        BytesMut,
        BytesPool,
    },
};

mod integer;
mod combinators;

#[cfg(test)]
mod tests;

pub trait WriteToBytesMut {
    fn write_to_bytes_mut(&self, bytes_mut: &mut BytesMut);
}

pub trait ReadFromBytes: Sized {
    type Error;

    fn read_from_bytes(bytes: Bytes) -> Result<(Self, Bytes), Self::Error>;
}

pub fn write<T>(blocks_pool: &BytesPool, value: &T) -> Bytes where T: WriteToBytesMut {
    let mut bytes_mut = blocks_pool.lend();
    value.write_to_bytes_mut(&mut bytes_mut);
    bytes_mut.freeze()
}

impl<'a, T> WriteToBytesMut for &'a T where T: WriteToBytesMut {
    fn write_to_bytes_mut(&self, bytes_mut: &mut BytesMut) {
        (*self).write_to_bytes_mut(bytes_mut);
    }
}
