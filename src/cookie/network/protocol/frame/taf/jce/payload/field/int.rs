use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::payload::field::{BYTE, HeadData, INT, JceType, JInt, SHORT, TYPE_ERR, ZERO_TAG};

impl JceType<JInt> for JInt {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        if *self < 32768 && *self >= -32768 { return (*self as i16).to_bytes(tag); }
        let mut b = HeadData::build(INT, tag, 4).format();
        b.put_i32(*self);
        b
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JInt {
        match r#type {
            BYTE => b.get_i8() as i32,
            SHORT => b.get_i16() as i32,
            INT => b.get_i32(),
            ZERO_TAG => 0,
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{INT, JceType, JInt, SHORT};

    #[test]
    fn to_bytes() { assert_eq!(114514_i32.to_bytes(0), vec![2, 0, 1, 191, 82]); }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JInt::from_bytes(&mut Bytes::from(vec![0, 1, 191, 82]), INT),
            114514_i32,
        );
    }

    #[test]
    fn to_bytes_short() { assert_eq!(1919_i32.to_bytes(0), vec![1, 7, 127]); }

    #[test]
    fn from_bytes_short() {
        assert_eq!(JInt::from_bytes(&mut Bytes::from(vec![7, 127]), SHORT), 1919_i32);
    }
}
