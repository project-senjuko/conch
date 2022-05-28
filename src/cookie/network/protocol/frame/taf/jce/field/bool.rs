use bytes::{Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::field::{BYTE, JBool, JceType, TYPE_ERR, ZERO_TAG};

impl JceType<JBool> for JBool {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        if *self { return 1_i8.to_bytes(tag); }
        0i8.to_bytes(tag)
    }

    fn from_bytes(_: &mut Bytes, r#type: u8) -> JBool {
        match r#type {
            BYTE => true,
            ZERO_TAG => false,
            _ => panic!("{}", TYPE_ERR)
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::field::{JBool, JceType, ZERO_TAG};

    #[test]
    fn to_bytes() { assert_eq!(true.to_bytes(0), vec![0, 1]); }

    #[test]
    fn from_bytes() {
        assert_eq!(JBool::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG), false);
    }
}
