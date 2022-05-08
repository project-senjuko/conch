use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldReader, FieldWriter};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;

use bytes::{BufMut, Bytes, BytesMut};
use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{STRING1, STRING4};

impl FieldBuild<String> for Field<String> {
    fn new(&HeadData { r#type, tag, .. }: &HeadData) -> Field<String> {
        Field { key: HeadData { r#type, tag, length: 0 }, value: String::new() }
    }

    fn with_value(&HeadData { r#type, tag, .. }: &HeadData, value: String) -> Field<String> {
        Field { key: HeadData { r#type, tag, length: value.len() as u32 }, value }
    }

    fn from_bytes(h: &HeadData, b: &mut Bytes) -> Field<String> {
        let mut a: Field<String> = Field::new(h);
        a.parse(b);
        a
    }
}

impl FieldReader for Field<String> {
    fn parse(&mut self, b: &mut Bytes) {
        let a = String::from_utf8(b.to_vec());
        self.key.length = b.len() as u32;
        self.value = a.unwrap_or_default();
    }
}

impl FieldWriter for Field<String> {
    fn format(&self) -> BytesMut {
        let l = self.value.len() as u32;
        let mut b = BytesMut::with_capacity((6 + l) as usize);

        if l <= 255 {
            b.put(HeadData { r#type: STRING1, tag: self.key.tag, length: l }.format());
            b.put_u8(l as u8);
        } else {
            b.put(HeadData { r#type: STRING4, tag: self.key.tag, length: l }.format());
            b.put_i32(l as i32);
        }
        b.put_slice(self.value.as_bytes());
        b
    }
}

#[cfg(test)]
mod tests {
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::{Field, FieldBuild, FieldWriter};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::{HeadData, ZERO_HEAD};
    use crate::cookie::network::protocol::frame::taf::jce::payload::field::r#type::{STRING1, STRING4};

    use bytes::Bytes;

    #[test]
    fn to_bytes1() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, String::from("好耶")).format().to_vec(),
            vec![6, 6, 229, 165, 189, 232, 128, 182],
        );
    }

    #[test]
    fn from_bytes1() {
        const H: HeadData = HeadData { r#type: STRING1, tag: 0, length: 6 };

        let a: Field<String> = Field::from_bytes(&H, &mut Bytes::from(vec![229, 165, 189, 232, 128, 182]));
        assert_eq!(a, Field { key: H, value: String::from("好耶") });
    }

    #[test]
    fn to_bytes2() {
        assert_eq!(
            Field::with_value(ZERO_HEAD, String::from(
                "好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！",
            )).format().to_vec(),
            vec![7, 0, 0, 1, 17, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129],
        );
    }

    #[test]
    fn from_bytes2() {
        const H: HeadData = HeadData { r#type: STRING4, tag: 0, length: 273 };

        let a: Field<String> = Field::from_bytes(&H, &mut Bytes::from(vec![229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 229, 165, 189, 232, 128, 182, 239, 188, 129, 230, 136, 145, 230, 152, 175, 229, 141, 131, 230, 169, 152, 233, 155, 171, 233, 156, 158, 239, 188, 129]));
        assert_eq!(a, Field { key: H, value: String::from("好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！好耶！好耶！我是千橘雫霞！") });
    }
}
