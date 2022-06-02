use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, HeadData, JceType, JSList, SIMPLE_LIST, TYPE_ERR};

impl JceType<JSList> for JSList {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(SIMPLE_LIST, tag, self.remaining() as u32).format(b);
        HeadData::new(BYTE, 0, 0).format(b);
        (self.remaining() as i32).to_bytes(b, 0);
        b.put(self.slice(..));
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> JSList {
        {
            let head = HeadData::parse(b);
            if head.tag != 0 || head.r#type != 0 { panic!("{}", TYPE_ERR) }
        }
        let (_, len) = HeadData::parse_ttl4(b);
        let a = b.slice(..len);
        b.advance(len);
        a
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{JceType, JSList, SIMPLE_LIST};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        Bytes::from(vec![1, 1, 4, 5, 1, 4]).to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![13, 0, 0, 6, 1, 1, 4, 5, 1, 4]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JSList::from_bytes(&mut Bytes::from(vec![0, 0, 6, 1, 1, 4, 5, 1, 4]), SIMPLE_LIST),
            Bytes::from(vec![1, 1, 4, 5, 1, 4]),
        );
    }
}
