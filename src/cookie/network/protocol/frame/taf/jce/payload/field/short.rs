use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::payload::field::{BYTE, HeadData, JceType, SHORT, TYPE_ERR, ZERO_TAG};

impl JceType<i16> for i16 {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        if *self < 128 && *self >= -128 { return (*self as i8).to_bytes(tag); }
        let mut b = HeadData::build(SHORT, tag, 2).format();
        b.put_i16(*self);
        b
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> i16 {
        match r#type {
            BYTE => b.get_i8() as i16,
            SHORT => b.get_i16(),
            ZERO_TAG => 0,
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{BYTE, JceType, JShort, SHORT};

    #[test]
    fn to_bytes() { assert_eq!(1919_i16.to_bytes(0).to_vec(), vec![1, 7, 127]); }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JShort::from_bytes(&mut Bytes::from(vec![7, 127]), SHORT),
            1919_i16,
        );
    }

    #[test]
    fn to_bytes_byte() { assert_eq!(114_i16.to_bytes(0).to_vec(), vec![0, 114]); }

    #[test]
    fn from_bytes_byte() {
        assert_eq!(
            JShort::from_bytes(&mut Bytes::from(vec![114]), BYTE),
            114_i16,
        );
    }
}
