use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::INT;

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i32> for Field<i32> {
    fn new(HeadData { tag, .. }: HeadData) -> Field<i32> {
        Field { key: HeadData { r#type: INT, tag }, value: 0i32 }
    }

    fn with_value(HeadData { tag, .. }: HeadData, value: i32) -> Field<i32> {
        Field { key: HeadData { r#type: INT, tag }, value }
    }

    fn from_bytes(h: HeadData, b: &mut Bytes) -> Field<i32> {
        let mut a: Field<i32> = Field::new(h);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<i32> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i32(); } }

impl FieldWriter for Field<i32> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(6);
        b.put(self.key.format());
        b.put_i32(self.value);
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::INT;

    use bytes::Bytes;

    #[test]
    fn to_bytes() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 114514_i32).format().to_vec(),
            vec![2, 0, 1, 191, 82],
        );
    }

    #[test]
    fn from_bytes() {
        let a: Field<i32> = Field::from_bytes(
            ZERO_HEAD,
            &mut Bytes::from(vec![0, 1, 191, 82]),
        );
        assert_eq!(a, Field { key: HeadData { r#type: INT, tag: 0 }, value: 114514_i32 });
    }
}
