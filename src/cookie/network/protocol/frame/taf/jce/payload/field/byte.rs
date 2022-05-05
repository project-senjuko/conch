use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::BYTE;

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<u8> for Field<u8> {
    fn new() -> Field<u8> { Field { key: ZERO_HEAD, value: 0u8 } }

    fn with_tag(tag: u8) -> Field<u8> { Field { key: HeadData { r#type: BYTE, tag }, value: 0u8 } }

    fn with_tv(tag: u8, value: u8) -> Field<u8> { Field { key: HeadData { r#type: BYTE, tag }, value } }

    fn from_bytes(tag: u8, b: &mut Bytes) -> Field<u8> {
        let mut a: Field<u8> = Field::with_tag(tag);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<u8> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_u8(); } }

impl FieldWriter for Field<u8> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(3);
        b.put(self.key.format());
        b.put_u8(self.value);
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::BYTE;

    use bytes::Bytes;

    #[test]
    fn to_bytes() { assert_eq!(Field::with_tv(0, 114_u8).format().to_vec(), vec![0, 114]); }

    #[test]
    fn from_bytes() {
        let a: Field<u8> = Field::from_bytes(0, &mut Bytes::from(vec![114]));
        assert_eq!(a, Field { key: HeadData { r#type: BYTE, tag: 0 }, value: 114_u8 });
    }
}
