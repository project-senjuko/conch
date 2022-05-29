use bytes::{BufMut, Bytes, BytesMut};

use super::{HeadData, JceStruct, JceType, STRUCT_BEGIN, STRUCT_END, TYPE_ERR};

impl<T: JceStruct<T>> JceType<T> for T {
    fn to_bytes(&self, tag: u8) -> BytesMut {
        let mut b = HeadData::new(STRUCT_BEGIN, tag, 0).format();
        b.put(self.s_to_bytes());
        b.put(HeadData::new(STRUCT_END, 0, 0).format());
        b
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> T {
        let a = T::s_from_bytes(T::init(), b);
        {
            let head = HeadData::parse(b);
            if head.tag != 0 || head.r#type != STRUCT_END { panic!("{}", TYPE_ERR) }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::super::{HeadData, JceStruct, JceType, STRING1, STRUCT_BEGIN};

    #[derive(PartialEq, Debug)]
    struct Q {
        name: String,
    }

    impl JceStruct<Q> for Q {
        fn s_to_bytes(&self) -> BytesMut { self.name.to_bytes(0) }

        fn s_from_bytes(mut self, b: &mut Bytes) -> Q {
            {
                let _ = HeadData::parse(b);
                self.name = String::from_bytes(b, STRING1);
            }
            self
        }

        fn init() -> Q { Q { name: String::new() } }
    }

    #[test]
    fn to_bytes() {
        assert_eq!(
            Q { name: String::from("千") }.to_bytes(0),
            vec![10, 6, 3, 229, 141, 131, 11],
        );
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            Q::from_bytes(&mut Bytes::from(vec![6, 3, 229, 141, 131, 11]), STRUCT_BEGIN),
            Q { name: String::from("千") },
        );
    }
}
