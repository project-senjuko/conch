////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::hash::Hash;

use bytes::{Buf, Bytes, BytesMut};

use super::{HeadData, JceFieldErr, JceKindReader, JceKindWriter, JMap, MAP};

impl<T, U> JceKindReader for JMap<T, U>
    where T: JceKindReader<T=T> + Eq + Hash,
          U: JceKindReader<T=U>
{
    type T = JMap<T, U>;
    fn from_bytes(b: &mut Bytes, _: u8) -> Result<Self::T, JceFieldErr> {
        let len = HeadData::parse_ttl4(b)?;
        let mut map: HashMap<T, U> = HashMap::with_capacity(b.remaining());
        {
            let mut i = 0;
            while i < len {
                let kh = HeadData::parse(b);
                let k = T::from_bytes(b, kh.r#type);
                let vh = HeadData::parse(b);
                let v = U::from_bytes(b, vh.r#type);
                map.insert(k?, v?);
                i += 1;
            }
        }
        Ok(map)
    }
}

impl<T, U> JceKindWriter for JMap<T, U>
    where T: JceKindWriter + Eq + Hash,
          U: JceKindWriter
{
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(MAP, tag).format(b, 0);
        (self.len() as i32).to_bytes(b, 0);
        for (k, v) in self.iter() {
            k.to_bytes(b, 0);
            v.to_bytes(b, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::super::{JByte, JceKindReader, JceKindWriter, JMap, JString, MAP};

    #[test]
    fn to_bytes() {
        let mut h: JMap<i8, String> = JMap::new();
        let mut b = BytesMut::new();

        h.insert(0, String::from("せんこさん大好き"));
        h.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![8, 0, 1, 12, 22, 24, 227, 129, 155, 227, 130, 147, 227, 129, 147, 227, 129, 149, 227, 130, 147, 229, 164, 167, 229, 165, 189, 227, 129, 141]);
    }

    #[test]
    fn from_bytes() {
        let mut h: JMap<i8, String> = JMap::new();
        h.insert(0, String::from("せんこさん"));
        h.insert(1, String::from("大好き"));
        assert_eq!(h, JMap::from_bytes(
            &mut Bytes::from(vec![0, 2, 12, 22, 15, 227, 129, 155, 227, 130, 147, 227, 129, 147, 227, 129, 149, 227, 130, 147, 0, 1, 22, 9, 229, 164, 167, 229, 165, 189, 227, 129, 141]),
            MAP,
        ).unwrap() as JMap<JByte, JString>);
    }
}
