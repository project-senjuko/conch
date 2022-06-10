use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, HeadData, INT, JceType, JLong, LONG, SHORT, TYPE_ERR, ZERO_TAG};

impl JceType<JLong> for JLong {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self < 2147483648 && *self >= -2147483648 { return (*self as i32).to_bytes(b, tag); }
        HeadData::new(LONG, tag).format(b, 8);
        b.put_i64(*self);
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JLong {
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
    use bytes::{Bytes, BytesMut};

    use super::{INT, JceType, JLong, LONG};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        1145141919810_i64.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![3, 0, 0, 1, 10, 159, 199, 0, 66]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![0, 0, 1, 10, 159, 199, 0, 66]), LONG),
            1145141919810_i64,
        );
    }

    #[test]
    fn to_bytes_int() {
        let mut b = BytesMut::new();
        114514_i64.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![2, 0, 1, 191, 82]);
    }

    #[test]
    fn from_bytes_int() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![0, 1, 191, 82]), INT),
            114514_i64,
        );
    }
}
