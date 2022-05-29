use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, HeadData, JByte, JceType, TYPE_ERR, ZERO_TAG};

impl JceType<JByte> for JByte {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        if *self == 0 { return HeadData::new(ZERO_TAG, tag, 0).format(); }
        let mut b = HeadData::new(BYTE, tag, 1).format();
        b.put_i8(*self);
        b
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JByte {
        match r#type {
            BYTE => b.get_i8(),
            ZERO_TAG => 0,
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::{BYTE, JByte, JceType, ZERO_TAG};

    #[test]
    fn to_bytes() { assert_eq!(114_i8.to_bytes(0), vec![0, 114]); }

    #[test]
    fn from_bytes() {
        assert_eq!(JByte::from_bytes(&mut Bytes::from(vec![114]), BYTE), 114_i8);
    }

    #[test]
    fn to_bytes_zero() { assert_eq!(0_i8.to_bytes(0), vec![12]); }

    #[test]
    fn from_bytes_zero() {
        assert_eq!(JByte::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG), 0_i8);
    }
}
