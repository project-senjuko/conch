use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, HeadData, INT, JceType, JInt, SHORT, TYPE_ERR, ZERO_TAG};

impl JceType<JInt> for JInt {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self < 32768 && *self >= -32768 { return (*self as i16).to_bytes(b, tag); }
        HeadData::new(INT, tag).format(b, 4);
        b.put_i32(*self);
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
    use bytes::{Bytes, BytesMut};

    use super::{INT, JceType, JInt, SHORT};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        114514_i32.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![2, 0, 1, 191, 82]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JInt::from_bytes(&mut Bytes::from(vec![0, 1, 191, 82]), INT),
            114514_i32,
        );
    }

    #[test]
    fn to_bytes_short() {
        let mut b = BytesMut::new();
        1919_i32.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![1, 7, 127]);
    }

    #[test]
    fn from_bytes_short() {
        assert_eq!(JInt::from_bytes(&mut Bytes::from(vec![7, 127]), SHORT), 1919_i32);
    }
}
