use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::field::{FLOAT, HeadData, JceType, JFloat, TYPE_ERR};

impl JceType<JFloat> for JFloat {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        let mut b = HeadData::new(FLOAT, tag, 4).format();
        b.put_f32(*self);
        b
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JFloat {
        match r#type {
            FLOAT => b.get_f32(),
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::field::{FLOAT, JceType, JFloat};

    #[test]
    fn to_bytes() { assert_eq!(11.4_f32.to_bytes(0), vec![4, 65, 54, 102, 102]); }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JFloat::from_bytes(&mut Bytes::from(vec![65, 54, 102, 102]), FLOAT),
            11.4_f32,
        );
    }
}
