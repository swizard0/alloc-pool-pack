use alloc_pool::{
    bytes::{
        BytesPool,
    },
};

use crate::{
    ReadFromBytes,
    WriteToBytesMut,
};

#[test]
fn serialize_deserialize_two_integers() {
    let bytes_pool = BytesPool::new();
    let mut bytes_mut = bytes_pool.lend();

    let a = 77_u32;
    let b = 144_u64;
    a.write_to_bytes_mut(&mut bytes_mut);
    b.write_to_bytes_mut(&mut bytes_mut);

    let bytes = bytes_mut.freeze();
    let (aa, bytes) = u32::read_from_bytes(bytes).unwrap();
    let (bb, bytes) = u64::read_from_bytes(bytes).unwrap();
    assert_eq!(a, aa);
    assert_eq!(b, bb);
    assert_eq!(bytes.len(), 0);
}
