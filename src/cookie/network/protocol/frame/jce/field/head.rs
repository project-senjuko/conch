use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, DOUBLE, FLOAT, INT, JceType, JInt, LIST, LONG, MAP, SHORT, SIMPLE_LIST, STRING1, STRING4, STRUCT_BEGIN, STRUCT_END, TYPE_ERR, ZERO_TAG};

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

impl HeadData {
    pub fn parse_ttl4(b: &mut Bytes) -> usize {
        let head = HeadData::parse(b);
        if head.tag != 0 { panic!("{}", TYPE_ERR) }
        JInt::from_bytes(b, head.r#type) as usize
    }

    pub fn skip_value(&self, b: &mut Bytes) {
        match self.r#type {
            BYTE => b.advance(1),
            SHORT => b.advance(2),
            INT => b.advance(4),
            LONG => b.advance(8),
            FLOAT => b.advance(4),
            DOUBLE => b.advance(8),
            STRING1 => {
                let l = b.get_u8() as usize;
                b.advance(l);
            }
            STRING4 => {
                let l = b.get_i32() as usize;
                b.advance(l);
            }
            MAP => {
                let len = HeadData::parse_ttl4(b);
                let mut i = 0;
                while i < len {
                    HeadData::parse(b).skip_value(b); // K
                    HeadData::parse(b).skip_value(b); // V
                    i += 1;
                }
            }
            LIST => {
                let len = HeadData::parse_ttl4(b);
                let mut i = 0;
                while i < len {
                    HeadData::parse(b).skip_value(b);
                    i += 1;
                }
            }
            STRUCT_BEGIN => {
                let mut h = HeadData::parse(b);
                while h.r#type != STRUCT_END {
                    h.skip_value(b);
                    h = HeadData::parse(b);
                }
            }
            STRUCT_END => {}
            ZERO_TAG => {}
            SIMPLE_LIST => {
                let len = HeadData::parse_ttl4(b);
                b.advance(1 + len); // 1: 0 type 0 tag head
            }
            _ => panic!("{}", TYPE_ERR),
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
    fn parse() {
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![0])), A);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![1])), B);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![33])), C);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![130])), D);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![244, 24])), E);
    }

    #[test]
    fn format() {
        let mut b = BytesMut::new();
        A.format(&mut b);
        assert_eq!(b.to_vec(), vec![0]);

        let mut b = BytesMut::new();
        B.format(&mut b);
        assert_eq!(b.to_vec(), vec![1]);

        let mut b = BytesMut::new();
        C.format(&mut b);
        assert_eq!(b.to_vec(), vec![33]);

        let mut b = BytesMut::new();
        D.format(&mut b);
        assert_eq!(b.to_vec(), vec![130]);

        let mut b = BytesMut::new();
        E.format(&mut b);
        assert_eq!(b.to_vec(), vec![244, 24]);
    }
}
