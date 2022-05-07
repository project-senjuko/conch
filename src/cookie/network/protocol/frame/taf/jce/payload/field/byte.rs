use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::BYTE;

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i8> for Field<i8> {
    fn new(HeadData { tag, .. }: HeadData) -> Field<i8> {
        Field { key: HeadData { r#type: BYTE, tag, length: 1 }, value: 0i8 }
    }

    fn with_value(HeadData { tag, .. }: HeadData, value: i8) -> Field<i8> {
        Field { key: HeadData { r#type: BYTE, tag, length: 1 }, value }
    }

    fn from_bytes(h: HeadData, b: &mut Bytes) -> Field<i8> {
        let mut a: Field<i8> = Field::new(h);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<i8> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i8(); } }

impl FieldWriter for Field<i8> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(3);
        b.put(self.key.format());
        b.put_i8(self.value);
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::BYTE;

    use bytes::Bytes;

    #[test]
    fn to_bytes() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 114_i8).format().to_vec(),
            vec![0, 114],
        );
    }

    #[test]
    fn from_bytes() {
        let a: Field<i8> = Field::from_bytes(ZERO_HEAD, &mut Bytes::from(vec![114]));
        assert_eq!(a, Field { key: HeadData { r#type: BYTE, tag: 0, length: 1 }, value: 114_i8 });
    }
}
