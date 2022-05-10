use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter, HeadData, TYPE_ERR};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{BYTE, ZERO_TAG};

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i8> for Field<i8> {
    fn new(&HeadData { tag, .. }: &HeadData) -> Field<i8> {
        Field { key: HeadData { r#type: BYTE, tag, length: 1 }, value: 0i8 }
    }

    fn with_head(h: &HeadData) -> Field<i8> { Field { key: h.clone(), value: 0i8 } }

    fn with_value(&HeadData { tag, .. }: &HeadData, value: i8) -> Field<i8> {
        Field { key: HeadData { r#type: BYTE, tag, length: 1 }, value }
    }

    fn from_bytes(h: &HeadData, b: &mut Bytes) -> Field<i8> {
        let mut a: Field<i8> = Field::with_head(h);
        match h.r#type {
            BYTE => a.parse(b),
            ZERO_TAG => {}
            _ => panic!("{}", TYPE_ERR),
        }
        a
    }
}

impl FieldReader for Field<i8> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i8(); } }

impl FieldWriter for Field<i8> {
    fn format(&self) -> BytesMut {
        let mut b = BytesMut::new();
        if self.value == 0 { // ZERO_TAG
            b.reserve(2);
            b.put(HeadData { r#type: ZERO_TAG, tag: self.key.tag, length: 0 }.format());
        } else { // BYTE
            b.reserve(3);
            b.put(self.key.format());
            b.put_i8(self.value);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter, HeadData};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::ZERO_HEAD;
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{BYTE, ZERO_TAG};

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
        const H: HeadData = HeadData { r#type: BYTE, tag: 0, length: 1 };

        let a: Field<i8> = Field::from_bytes(&H, &mut Bytes::from(vec![114]));
        assert_eq!(a, Field { key: H, value: 114_i8 });
    }

    #[test]
    fn to_bytes_zero() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 0_i8).format().to_vec(), vec![12],
        );
    }

    #[test]
    fn from_bytes_zero() {
        const H: HeadData = HeadData { r#type: ZERO_TAG, tag: 0, length: 0 };

        let a: Field<i8> = Field::from_bytes(&H, &mut Bytes::from(vec![]));
        assert_eq!(a, Field { key: H, value: 0_i8 });
    }
}
