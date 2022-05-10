use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter, HeadData, TYPE_ERR};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{BYTE, SHORT, ZERO_TAG};

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i16> for Field<i16> {
    fn new(&HeadData { tag, .. }: &HeadData) -> Field<i16> {
        Field { key: HeadData { r#type: SHORT, tag, length: 2 }, value: 0i16 }
    }

    fn with_head(h: &HeadData) -> Field<i16> { Field { key: h.clone(), value: 0i16 } }

    fn with_value(&HeadData { tag, .. }: &HeadData, value: i16) -> Field<i16> {
        Field { key: HeadData { r#type: SHORT, tag, length: 2 }, value }
    }

    fn from_bytes(h: &HeadData, b: &mut Bytes) -> Field<i16> {
        let mut a: Field<i16> = Field::with_head(h);
        match h.r#type {
            BYTE => a.value = b.get_i8() as i16,
            SHORT => a.parse(b),
            ZERO_TAG => {}
            _ => panic!("{}", TYPE_ERR),
        }
        a
    }
}

impl FieldReader for Field<i16> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i16(); } }

impl FieldWriter for Field<i16> {
    fn format(&self) -> BytesMut {
        if self.value < 128 && self.value >= -128 {
            let a = Field::with_value(&self.key, self.value as i8);
            a.format()
        } else {
            let mut b = BytesMut::with_capacity(4);
            b.put(self.key.format());
            b.put_i16(self.value);
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter, HeadData};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::ZERO_HEAD;
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{BYTE, SHORT};

    use bytes::Bytes;

    #[test]
    fn to_bytes() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 1919_i16).format().to_vec(),
            vec![1, 7, 127],
        );
    }

    #[test]
    fn from_bytes() {
        const H: HeadData = HeadData { r#type: SHORT, tag: 0, length: 2 };

        let a: Field<i16> = Field::from_bytes(&H, &mut Bytes::from(vec![7, 127]));
        assert_eq!(a, Field { key: H, value: 1919_i16 });
    }

    #[test]
    fn to_bytes_byte() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 114_i16).format().to_vec(),
            vec![0, 114],
        );
    }

    #[test]
    fn from_bytes_byte() {
        const H: HeadData = HeadData { r#type: BYTE, tag: 0, length: 1 };

        let a: Field<i16> = Field::from_bytes(&H, &mut Bytes::from(vec![114]));
        assert_eq!(a, Field { key: H, value: 114_i16 });
    }
}
