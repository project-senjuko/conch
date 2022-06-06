use bytes::{BufMut, BytesMut};

use super::field::JceType;

pub struct JceWriter {
    b: BytesMut,
    tag: u8,
}

impl JceWriter {
    pub fn new() -> JceWriter { JceWriter { b: BytesMut::new(), tag: 0 } }

    pub fn with_tag(tag: u8) -> JceWriter { JceWriter { b: BytesMut::new(), tag } }
}

impl JceWriter {
    pub fn advance(&mut self, cnt: u8) { self.tag += cnt; }

    pub fn put<T: JceType<T>>(&mut self, t: &T) {
        t.to_bytes(&mut self.b, self.tag);
        self.advance(1);
    }

    pub fn to_bytes(self, b: &mut BytesMut) { b.put(self.b); }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::JceWriter;

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        let mut w = JceWriter::with_tag(1);
        w.put(&1);
        w.put(&String::from("千橘橘"));
        w.to_bytes(&mut b);
        assert_eq!(b.to_vec(), vec![16, 1, 38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152]);
    }
}
