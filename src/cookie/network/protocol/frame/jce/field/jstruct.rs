use bytes::{Bytes, BytesMut};

use super::{HeadData, JceStruct, JceType, STRUCT_BEGIN, STRUCT_END, TYPE_ERR};

impl<T: JceStruct<T>> JceType<T> for T {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(STRUCT_BEGIN, tag).format(b, 0);
        self.s_to_bytes(b);
        HeadData::new(STRUCT_END, 0).format(b, 0);
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> T {
        let mut t = T::init();
        t.s_from_bytes(b);
        {
            let head = HeadData::parse(b);
            if head.tag != 0 || head.r#type != STRUCT_END { panic!("{}", TYPE_ERR) }
        }
        t
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
        fn s_to_bytes(&self, b: &mut BytesMut) { self.name.to_bytes(b, 0); }

        fn s_from_bytes(&mut self, b: &mut Bytes) {
            let _ = HeadData::parse(b);
            self.name = String::from_bytes(b, STRING1);
        }

        fn init() -> Q { Q { name: String::new() } }
    }

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        Q { name: String::from("千") }.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![10, 6, 3, 229, 141, 131, 11]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            Q::from_bytes(&mut Bytes::from(vec![6, 3, 229, 141, 131, 11]), STRUCT_BEGIN),
            Q { name: String::from("千") },
        );
    }
}
