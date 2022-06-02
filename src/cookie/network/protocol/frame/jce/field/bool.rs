use bytes::{Buf, Bytes, BytesMut};

use super::{BOOL, JBool, JceType, TYPE_ERR, ZERO_TAG};

impl JceType<JBool> for JBool {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self { return 1_i8.to_bytes(b, tag); }
        0i8.to_bytes(b, tag);
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JBool {
        match r#type {
            BOOL => {
                b.advance(1);
                true
            }
            ZERO_TAG => false,
            _ => panic!("{}", TYPE_ERR)
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{JBool, JceType, ZERO_TAG};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        true.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![0, 1]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(JBool::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG), false);
    }
}
