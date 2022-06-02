use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{FLOAT, HeadData, JceType, JFloat, TYPE_ERR};

impl JceType<JFloat> for JFloat {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(FLOAT, tag, 4).format(b);
        b.put_f32(*self);
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
    use bytes::{Bytes, BytesMut};

    use super::{FLOAT, JceType, JFloat};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        11.4_f32.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![4, 65, 54, 102, 102]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JFloat::from_bytes(&mut Bytes::from(vec![65, 54, 102, 102]), FLOAT),
            11.4_f32,
        );
    }
}
