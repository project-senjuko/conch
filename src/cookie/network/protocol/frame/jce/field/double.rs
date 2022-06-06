use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{DOUBLE, HeadData, JceType, JDouble};

impl JceType<JDouble> for JDouble {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(DOUBLE, tag, 8).format(b);
        b.put_f64(*self);
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> JDouble { b.get_f64() }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{DOUBLE, JceType, JDouble};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        114.5141919810_f64.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![5, 64, 92, 160, 232, 133, 123, 144, 171]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JDouble::from_bytes(&mut Bytes::from(vec![64, 92, 160, 232, 133, 123, 144, 171]), DOUBLE),
            114.5141919810_f64,
        );
    }
}
