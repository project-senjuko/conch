use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::SHORT;

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i16> for Field<i16> {
    fn new(HeadData { tag, .. }: HeadData) -> Field<i16> {
        Field { key: HeadData { r#type: SHORT, tag }, value: 0i16 }
    }

    fn with_value(HeadData { tag, .. }: HeadData, value: i16) -> Field<i16> {
        Field { key: HeadData { r#type: SHORT, tag }, value }
    }

    fn from_bytes(h: HeadData, b: &mut Bytes) -> Field<i16> {
        let mut a: Field<i16> = Field::new(h);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<i16> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i16(); } }

impl FieldWriter for Field<i16> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(4);
        b.put(self.key.format());
        b.put_i16(self.value);
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::SHORT;

    use bytes::Bytes;

    #[test]
    fn to_bytes() {
        assert_eq!(
            Field::with_value(ZERO_HEAD,
                              1919_i16).format().to_vec(), vec![1, 7, 127],
        );
    }

    #[test]
    fn from_bytes() {
        let a: Field<i16> = Field::from_bytes(
            ZERO_HEAD,
            &mut Bytes::from(vec![7, 127]),
        );
        assert_eq!(a, Field { key: HeadData { r#type: SHORT, tag: 0 }, value: 1919_i16 });
    }
}
