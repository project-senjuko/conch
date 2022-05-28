use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::field::{BYTE, HeadData, JceType, JSList, SIMPLE_LIST, TYPE_ERR};

impl JceType<JSList> for JSList {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        let mut b = HeadData::new(SIMPLE_LIST, tag, self.remaining() as u32).format();
        b.put(HeadData::new(BYTE, tag, 0).format());
        b.put((self.remaining() as i32).to_bytes(0));
        b.put(self.slice(..));
        b
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
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::field::{JceType, JSList, SIMPLE_LIST};

    #[test]
    fn to_bytes() {
        assert_eq!(
            Bytes::from(vec![1, 1, 4, 5, 1, 4]).to_bytes(0),
            vec![13, 0, 0, 6, 1, 1, 4, 5, 1, 4],
        );
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JSList::from_bytes(&mut Bytes::from(vec![0, 0, 6, 1, 1, 4, 5, 1, 4]), SIMPLE_LIST),
            Bytes::from(vec![1, 1, 4, 5, 1, 4]),
        );
    }
}
