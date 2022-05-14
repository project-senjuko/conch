use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::payload::field::{DOUBLE, HeadData, JceType, TYPE_ERR};

impl JceType<f64> for f64 {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        let mut b = HeadData::build(DOUBLE, tag, 8).format();
        b.put_f64(*self);
        b
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> f64 {
        match r#type {
            DOUBLE => b.get_f64(),
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{DOUBLE, JceType, JDouble};

    #[test]
    fn to_bytes() {
        assert_eq!(
            114.5141919810_f64.to_bytes(0).to_vec(),
            vec![5, 64, 92, 160, 232, 133, 123, 144, 171],
        );
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JDouble::from_bytes(&mut Bytes::from(vec![64, 92, 160, 232, 133, 123, 144, 171]), DOUBLE),
            114.5141919810_f64,
        );
    }
}
