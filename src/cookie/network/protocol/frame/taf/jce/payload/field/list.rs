use bytes::{BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::payload::field::{HeadData, JceType, JInt, JList, LIST, TYPE_ERR};

impl<T: JceType<T>> JceType<JList<T>> for JList<T> {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        let mut b = HeadData::build(LIST, tag, self.capacity() as u32).format();
        b.put((self.len() as i32).to_bytes(0));
        for v in self.iter() { b.put(v.to_bytes(0)) }
        b
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> JList<T> {
        let len = {
            let head = HeadData::parse(b);
            if head.tag != 0 { panic!("{}", TYPE_ERR) }
            JInt::from_bytes(b, head.r#type) as u32
        };
        let mut vec: Vec<T> = Vec::new();
        {
            let mut i = 0;
            while i < len {
                let vh = HeadData::parse(b);
                vec.push(T::from_bytes(b, vh.r#type));
                i += 1;
            }
        };
        vec
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{JceType, JList, LIST};

    #[test]
    fn to_bytes() {
        assert_eq!(
            vec![String::from("千橘"), String::from("雫霞")].to_bytes(0),
            vec![9, 0, 2, 6, 6, 229, 141, 131, 230, 169, 152, 6, 6, 233, 155, 171, 233, 156, 158],
        );
    }

    #[test]
    fn from_bytes() {
        let a: JList<String> = JList::from_bytes(&mut Bytes::from(vec![0, 2, 6, 6, 229, 141, 131, 230, 169, 152, 6, 6, 233, 155, 171, 233, 156, 158]), LIST);
        assert_eq!(a, vec![String::from("千橘"), String::from("雫霞")])
    }
}
