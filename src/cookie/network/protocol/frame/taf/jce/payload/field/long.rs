use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter, HeadData, TYPE_ERR};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{BYTE, INT, LONG, SHORT, ZERO_TAG};

use bytes::{Buf, BufMut, Bytes, BytesMut};

impl FieldBuild<i64> for Field<i64> {
    fn new(&HeadData { tag, .. }: &HeadData) -> Field<i64> {
        Field { key: HeadData { r#type: LONG, tag, length: 8 }, value: 0i64 }
    }

    fn with_head(h: &HeadData) -> Field<i64> { Field { key: h.clone(), value: 0i64 } }

    fn with_value(&HeadData { tag, .. }: &HeadData, value: i64) -> Field<i64> {
        Field { key: HeadData { r#type: LONG, tag, length: 8 }, value }
    }

    fn from_bytes(h: &HeadData, b: &mut Bytes) -> Field<i64> {
        let mut a: Field<i64> = Field::with_head(h);
        match h.r#type {
            BYTE => a.value = b.get_i8() as i64,
            SHORT => a.value = b.get_i16() as i64,
            INT => a.value = b.get_i32() as i64,
            LONG => a.parse(b),
            ZERO_TAG => {}
            _ => panic!("{}", TYPE_ERR),
        }
        a
    }
}

impl FieldReader for Field<i64> { fn parse(&mut self, b: &mut Bytes) { self.value = b.get_i64(); } }

impl FieldWriter for Field<i64> {
    fn format(&self) -> BytesMut {
        if self.value < 2147483648 && self.value >= -2147483648 {
            let a = Field::with_value(&self.key, self.value as i32);
            a.format()
        } else {
            let mut b = BytesMut::with_capacity(10);
            b.put(self.key.format());
            b.put_i64(self.value);
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter, HeadData};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::ZERO_HEAD;
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{INT, LONG};

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
        const H: HeadData = HeadData { r#type: LONG, tag: 0, length: 8 };

        let a: Field<i64> = Field::from_bytes(&H, &mut Bytes::from(vec![0, 0, 1, 10, 159, 199, 0, 66]));
        assert_eq!(a, Field { key: H, value: 1145141919810_i64 });
    }

    #[test]
    fn to_bytes_int() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, 114514_i64).format().to_vec(),
            vec![2, 0, 1, 191, 82],
        );
    }

    #[test]
    fn from_bytes_int() {
        const H: HeadData = HeadData { r#type: INT, tag: 0, length: 4 };

        let a: Field<i64> = Field::from_bytes(&H, &mut Bytes::from(vec![0, 1, 191, 82]));
        assert_eq!(a, Field { key: H, value: 114514_i64 });
    }
}
