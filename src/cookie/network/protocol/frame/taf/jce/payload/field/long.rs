use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::payload::field::{BYTE, HeadData, INT, JceType, LONG, SHORT, TYPE_ERR, ZERO_TAG};

impl JceType<i64> for i64 {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        if *self < 2147483648 && *self >= -2147483648 { return (*self as i32).to_bytes(tag); }
        let mut b = HeadData::build(LONG, tag, 8).format();
        b.put_i64(*self);
        b
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> i64 {
        match r#type {
            BYTE => b.get_i8() as i64,
            SHORT => b.get_i16() as i64,
            INT => b.get_i32() as i64,
            LONG => b.get_i64(),
            ZERO_TAG => 0,
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{INT, JceType, JLong, LONG};

    #[test]
    fn to_bytes() {
        assert_eq!(1145141919810_i64.to_bytes(0).to_vec(), vec![3, 0, 0, 1, 10, 159, 199, 0, 66]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![0, 0, 1, 10, 159, 199, 0, 66]), LONG),
            1145141919810_i64,
        );
    }

    #[test]
    fn to_bytes_int() { assert_eq!(114514_i64.to_bytes(0).to_vec(), vec![2, 0, 1, 191, 82]); }

    #[test]
    fn from_bytes_int() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![0, 1, 191, 82]), INT),
            114514_i64,
        );
    }
}
