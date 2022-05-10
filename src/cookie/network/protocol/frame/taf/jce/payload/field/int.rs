use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter, HeadData, TYPE_ERR};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{BYTE, INT, SHORT, ZERO_TAG};

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i32> for Field<i32> {
    fn new(&HeadData { tag, .. }: &HeadData) -> Field<i32> {
        Field { key: HeadData { r#type: INT, tag, length: 4 }, value: 0i32 }
    }

    fn with_head(h: &HeadData) -> Field<i32> { Field { key: h.clone(), value: 0i32 } }

    fn with_value(&HeadData { tag, .. }: &HeadData, value: i32) -> Field<i32> {
        Field { key: HeadData { r#type: INT, tag, length: 4 }, value }
    }

    fn from_bytes(h: &HeadData, b: &mut Bytes) -> Field<i32> {
        let mut a: Field<i32> = Field::with_head(h);
        match h.r#type {
            BYTE => a.value = b.get_i8() as i32,
            SHORT => a.value = b.get_i16() as i32,
            INT => a.parse(b),
            ZERO_TAG => {}
            _ => panic!("{}", TYPE_ERR),
        }
        a
    }
}

impl FieldReader for Field<i32> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i32(); } }

impl FieldWriter for Field<i32> {
    fn format(&self) -> BytesMut {
        if self.value < 32768 && self.value >= -32768 {
            let a = Field::with_value(&self.key, self.value as i16);
            a.format()
        } else {
            let mut b = BytesMut::with_capacity(6);
            b.put(self.key.format());
            b.put_i32(self.value);
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter, HeadData};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::ZERO_HEAD;
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{INT, SHORT};

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
        const H: HeadData = HeadData { r#type: INT, tag: 0, length: 4 };

        let a: Field<i32> = Field::from_bytes(&H, &mut Bytes::from(vec![0, 1, 191, 82]));
        assert_eq!(a, Field { key: H, value: 114514_i32 });
    }


    #[test]
    fn to_bytes_short() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 1919_i32).format().to_vec(),
            vec![1, 7, 127],
        );
    }

    #[test]
    fn from_bytes_short() {
        const H: HeadData = HeadData { r#type: SHORT, tag: 0, length: 2 };

        let a: Field<i32> = Field::from_bytes(&H, &mut Bytes::from(vec![7, 127]));
        assert_eq!(a, Field { key: H, value: 1919_i32 });
    }
}
