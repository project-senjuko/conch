use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::LONG;

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i64> for Field<i64> {
    fn new(&HeadData { tag, .. }: &HeadData) -> Field<i64> {
        Field { key: HeadData { r#type: LONG, tag, length: 8 }, value: 0i64 }
    }

    fn with_value(&HeadData { tag, .. }: &HeadData, value: i64) -> Field<i64> {
        Field { key: HeadData { r#type: LONG, tag, length: 8 }, value }
    }

    fn from_bytes(h: &HeadData, b: &mut Bytes) -> Field<i64> {
        let mut a: Field<i64> = Field::new(h);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<i64> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i64(); } }

impl FieldWriter for Field<i64> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(10);
        b.put(self.key.format());
        b.put_i64(self.value);
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::LONG;

    use bytes::Bytes;

    #[test]
    fn to_bytes() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 1145141919810_i64).format().to_vec(),
            vec![3, 0, 0, 1, 10, 159, 199, 0, 66],
        );
    }

    #[test]
    fn from_bytes() {
        let a: Field<i64> = Field::from_bytes(
            ZERO_HEAD,
            &mut Bytes::from(vec![0, 0, 1, 10, 159, 199, 0, 66]),
        );
        assert_eq!(a, Field { key: HeadData { r#type: LONG, tag: 0, length: 8 }, value: 1145141919810_i64 });
    }
}
