use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::DOUBLE;

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<f64> for Field<f64> {
    fn new(&HeadData { tag, .. }: &HeadData) -> Field<f64> {
        Field { key: HeadData { r#type: DOUBLE, tag, length: 8 }, value: 0f64 }
    }

    fn with_value(&HeadData { tag, .. }: &HeadData, value: f64) -> Field<f64> {
        Field { key: HeadData { r#type: DOUBLE, tag, length: 8 }, value }
    }

    fn from_bytes(h: &HeadData, b: &mut Bytes) -> Field<f64> {
        let mut a: Field<f64> = Field::new(h);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<f64> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_f64(); } }

impl FieldWriter for Field<f64> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(10);
        b.put(self.key.format());
        b.put_f64(self.value);
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::DOUBLE;

    use bytes::Bytes;

    #[test]
    fn to_bytes() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 114.5141919810_f64).format().to_vec(),
            vec![5, 64, 92, 160, 232, 133, 123, 144, 171],
        );
    }

    #[test]
    fn from_bytes() {
        let a: Field<f64> = Field::from_bytes(
            ZERO_HEAD,
            &mut Bytes::from(vec![64, 92, 160, 232, 133, 123, 144, 171]),
        );
        assert_eq!(a, Field { key: HeadData { r#type: DOUBLE, tag: 0, length: 8 }, value: 114.5141919810_f64 });
    }
}
