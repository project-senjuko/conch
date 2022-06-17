use bytes::{Buf, Bytes, BytesMut};

use super::{HeadData, JceType, JList, LIST};

impl<T: JceType<T>> JceType<JList<T>> for JList<T> {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(LIST, tag).format(b, self.capacity());
        (self.len() as i32).to_bytes(b, 0);
        for v in self.iter() { v.to_bytes(b, 0) }
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> JList<T> {
        let len = HeadData::parse_ttl4(b);
        let mut vec: Vec<T> = Vec::with_capacity(b.remaining());
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
    use bytes::{Bytes, BytesMut};

    use super::{JceType, JList, LIST};
    use super::super::JString;

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        vec![String::from("千橘"), String::from("雫霞")].to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![9, 0, 2, 6, 6, 229, 141, 131, 230, 169, 152, 6, 6, 233, 155, 171, 233, 156, 158]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JList::from_bytes(&mut Bytes::from(vec![0, 2, 6, 6, 229, 141, 131, 230, 169, 152, 6, 6, 233, 155, 171, 233, 156, 158]), LIST) as JList<JString>,
            vec![String::from("千橘"), String::from("雫霞")],
        );
    }
}
