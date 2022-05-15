use std::collections::HashMap;
use std::hash::Hash;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::payload::field::{HeadData, JceType, JInt, JMap, MAP, TYPE_ERR};

impl<T: JceType<T> + Eq + Hash, U: JceType<U>> JceType<JMap<T, U>> for JMap<T, U> {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        let mut b = HeadData::build(MAP, tag, self.capacity() as u32).format();
        b.put((self.len() as i32).to_bytes(0));
        for (k, v) in self.iter() {
            b.put(k.to_bytes(0));
            b.put(v.to_bytes(1));
        }
        b
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> JMap<T, U> {
        let len = {
            let head = HeadData::parse(b);
            if head.tag != 0 { panic!("{}", TYPE_ERR) }
            JInt::from_bytes(b, head.r#type) as u32
        };
        let mut map: HashMap<T, U> = HashMap::with_capacity(b.remaining());
        {
            let mut i = 0;
            while i < len {
                let kh = HeadData::parse(b);
                let k = T::from_bytes(b, kh.r#type);
                let vh = HeadData::parse(b);
                let v = U::from_bytes(b, vh.r#type);
                map.insert(k, v);
                i += 1;
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{JceType, JMap, MAP};

    #[test]
    fn to_bytes() {
        let mut h: JMap<i8, String> = JMap::new();
        h.insert(0, String::from("せんこさん大好き"));
        assert_eq!(h.to_bytes(0), vec![8, 0, 1, 12, 22, 24, 227, 129, 155, 227, 130, 147, 227, 129, 147, 227, 129, 149, 227, 130, 147, 229, 164, 167, 229, 165, 189, 227, 129, 141]);
    }

    #[test]
    fn from_bytes() {
        let mut h: JMap<i8, String> = JMap::new();
        h.insert(0, String::from("せんこさん"));
        h.insert(1, String::from("大好き"));
        let a: JMap<i8, String> = JMap::from_bytes(&mut Bytes::from(vec![0, 2, 12, 22, 15, 227, 129, 155, 227, 130, 147, 227, 129, 147, 227, 129, 149, 227, 130, 147, 0, 1, 22, 9, 229, 164, 167, 229, 165, 189, 227, 129, 141]), MAP);
        assert_eq!(h, a);
    }
}
