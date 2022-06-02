use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{JceType, JInt, TYPE_ERR};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HeadData {
    pub r#type: u8,
    pub tag: u8,
    pub length: u32,
}

impl HeadData {
    pub fn new(r#type: u8, tag: u8, length: u32) -> HeadData { HeadData { r#type, tag, length } }

    pub fn parse(b: &mut Bytes) -> HeadData {
        let f = b.get_u8();
        let r#type = f & 15;
        let mut t = (f & 240) >> 4;

        if t == 15 {
            t = b.get_u8() & 255;
        }

        HeadData { r#type, tag: t, length: 0 }
    }

    pub fn parse_ttl4(b: &mut Bytes) -> (HeadData, usize) {
        let head = HeadData::parse(b);
        if head.tag != 0 { panic!("{}", TYPE_ERR) }
        (head, JInt::from_bytes(b, head.r#type) as usize)
    }

    pub fn format(&self, b: &mut BytesMut) {
        b.reserve(2 + self.length as usize);
        if self.tag <= 14 {
            b.put_u8(self.r#type | (self.tag << 4));
        } else {
            b.put_u8(self.r#type | 240);
            b.put_u8(self.tag);
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::HeadData;

    const A: HeadData = HeadData { r#type: 0, tag: 0, length: 0 };
    const B: HeadData = HeadData { r#type: 1, tag: 0, length: 0 };
    const C: HeadData = HeadData { r#type: 1, tag: 2, length: 0 };
    const D: HeadData = HeadData { r#type: 2, tag: 8, length: 0 };
    const E: HeadData = HeadData { r#type: 4, tag: 24, length: 0 };

    #[test]
    fn parse0() { assert_eq!(HeadData::parse(&mut Bytes::from(vec![0])), A); }

    #[test]
    fn parse1() { assert_eq!(HeadData::parse(&mut Bytes::from(vec![1])), B); }

    #[test]
    fn parse33() { assert_eq!(HeadData::parse(&mut Bytes::from(vec![33])), C); }

    #[test]
    fn parse130() { assert_eq!(HeadData::parse(&mut Bytes::from(vec![130])), D); }

    #[test]
    fn parse24424() { assert_eq!(HeadData::parse(&mut Bytes::from(vec![244, 24])), E); }

    #[test]
    fn format00() {
        let mut b = BytesMut::new();
        A.format(&mut b);
        assert_eq!(b.to_vec(), vec![0]);
    }

    #[test]
    fn format10() {
        let mut b = BytesMut::new();
        B.format(&mut b);
        assert_eq!(b.to_vec(), vec![1]);
    }

    #[test]
    fn format12() {
        let mut b = BytesMut::new();
        C.format(&mut b);
        assert_eq!(b.to_vec(), vec![33]);
    }

    #[test]
    fn format28() {
        let mut b = BytesMut::new();
        D.format(&mut b);
        assert_eq!(b.to_vec(), vec![130]);
    }

    #[test]
    fn format424() {
        let mut b = BytesMut::new();
        E.format(&mut b);
        assert_eq!(b.to_vec(), vec![244, 24]);
    }
}
