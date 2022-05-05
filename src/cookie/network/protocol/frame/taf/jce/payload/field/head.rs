use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(PartialEq, Debug)]
pub struct HeadData {
    pub r#type: u8,
    pub tag: u8,
}

pub const ZERO_HEAD: HeadData = HeadData { r#type: 0, tag: 0 };

impl HeadData {
    pub fn parse(b: &mut Bytes) -> HeadData {
        let f = b.get_u8();
        let mut t = (f & 240) >> 4;

        if t == 15 {
            t = b.get_u8() & 255;
        }
        HeadData { r#type: f & 15, tag: t }
    }

    pub fn format(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(2);
        if self.tag <= 14 {
            b.put_u8(self.r#type | (self.tag << 4));
        } else {
            b.put_u8(self.r#type | 240);
            b.put_u8(self.tag);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::HeadData;

    use bytes::Bytes;

    const A: HeadData = HeadData { r#type: 0, tag: 0 };
    const B: HeadData = HeadData { r#type: 1, tag: 0 };
    const C: HeadData = HeadData { r#type: 1, tag: 2 };
    const D: HeadData = HeadData { r#type: 2, tag: 8 };
    const E: HeadData = HeadData { r#type: 4, tag: 24 };

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
    fn format00() { assert_eq!(A.format().to_vec(), vec![0]); }

    #[test]
    fn format10() { assert_eq!(B.format().to_vec(), vec![1]); }

    #[test]
    fn format12() { assert_eq!(C.format().to_vec(), vec![33]); }

    #[test]
    fn format28() { assert_eq!(D.format().to_vec(), vec![130]); }

    #[test]
    fn format424() { assert_eq!(E.format().to_vec(), vec![244, 24]); }
}
