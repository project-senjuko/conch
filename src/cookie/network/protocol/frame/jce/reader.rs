use bytes::{Buf, Bytes};
use rustc_hash::FxHashMap;

use super::field::{HeadData, JceType};

pub struct JceReader {
    b: Bytes,
    tag: u8,
    cache: FxHashMap<u8, (HeadData, Bytes)>,
}

impl JceReader {
    pub fn new(b: Bytes) -> JceReader { JceReader { b, tag: 0, cache: FxHashMap::default() } }

    pub fn with_tag(b: Bytes, tag: u8) -> JceReader { JceReader { b, tag, cache: FxHashMap::default() } }
}

impl JceReader {
    #[inline(always)]
    pub fn advance(&mut self, cnt: u8) { self.tag += cnt; }

    #[inline(always)]
    pub fn get<T: JceType<T>>(&mut self) -> T {
        match self.get_optional() {
            Some(o) => o,
            None => panic!("Jce 要求必须的字段不存在"),
        }
    }

    pub fn get_optional<T: JceType<T>>(&mut self) -> Option<T> {
        let r = self._get_optional();
        self.advance(1);
        r
    }

    #[inline(always)]
    fn _get_optional<T: JceType<T>>(&mut self) -> Option<T> {
        if !self.b.has_remaining() {
            return self.cache.get(&self.tag).map(
                |o| T::from_bytes(&mut o.1.clone(), o.0.r#type),
            );
        }

        let mut h = HeadData::parse(&mut self.b);
        while h.tag != self.tag {
            let rb = self.b.clone();
            h.skip_value(&mut self.b);
            self.cache.insert(
                h.tag,
                (h, rb.slice(..rb.len() - self.b.remaining())),
            );

            if self.b.has_remaining() {
                h = HeadData::parse(&mut self.b);
            } else {
                return self._get_optional();
            }
        }
        Some(T::from_bytes(&mut self.b, h.r#type))
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::jce::field::{JByte, JString};

    use super::JceReader;

    #[test]
    fn get() {
        let mut r = JceReader::with_tag(Bytes::from(
            vec![16, 1, 38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152],
        ), 1);
        let num: JByte = r.get();
        let str: JString = r.get();

        assert_eq!(num, 1);
        assert_eq!(str, "千橘橘");
    }

    #[test]
    fn get_wild() {
        let mut r = JceReader::with_tag(Bytes::from(
            vec![38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152, 16, 1],
        ), 1);
        let num: JByte = r.get();
        let str: JString = r.get();

        assert_eq!(num, 1);
        assert_eq!(str, "千橘橘");
    }

    #[test]
    fn get_optional() {
        let mut r = JceReader::with_tag(Bytes::from(
            vec![38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152],
        ), 1);
        let num: Option<JByte> = r.get_optional();
        let str: JString = r.get();

        assert_eq!(num, None);
        assert_eq!(str, "千橘橘");
    }
}
