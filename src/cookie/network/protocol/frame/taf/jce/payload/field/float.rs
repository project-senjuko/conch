use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::FLOAT;

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<f32> for Field<f32> {
    fn new(HeadData { tag, .. }: HeadData) -> Field<f32> {
        Field { key: HeadData { r#type: FLOAT, tag }, value: 0f32 }
    }

    fn with_value(HeadData { tag, .. }: HeadData, value: f32) -> Field<f32> {
        Field { key: HeadData { r#type: FLOAT, tag }, value }
    }

    fn from_bytes(h: HeadData, b: &mut Bytes) -> Field<f32> {
        let mut a: Field<f32> = Field::new(h);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<f32> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_f32(); } }

impl FieldWriter for Field<f32> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(6);
        b.put(self.key.format());
        b.put_f32(self.value);
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::FLOAT;

    use bytes::Bytes;

    #[test]
    fn to_bytes() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 11.4_f32).format().to_vec(),
            vec![4, 65, 54, 102, 102],
        );
    }

    #[test]
    fn from_bytes() {
        let a: Field<f32> = Field::from_bytes(
            ZERO_HEAD,
            &mut Bytes::from(vec![65, 54, 102, 102]),
        );
        assert_eq!(a, Field { key: HeadData { r#type: FLOAT, tag: 0 }, value: 11.4_f32 });
    }
}
