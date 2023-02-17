////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, Bytes, BytesMut};

use super::{HeadData, JceFieldErr, JceKindReader, JceKindWriter, JList, LIST};

impl<T> JceKindReader for JList<T>
    where T: JceKindReader<T=T>
{
    type T = JList<T>;
    fn from_bytes(b: &mut Bytes, _: u8) -> Result<Self::T, JceFieldErr> {
        let len = HeadData::parse_ttl4(b)?;
        let mut vec: Vec<T> = Vec::with_capacity(b.remaining());
        {
            let mut i = 0;
            while i < len {
                let vh = HeadData::parse(b);
                vec.push(T::from_bytes(b, vh.r#type)?);
                i += 1;
            }
        };
        Ok(vec)
    }
}

impl<T> JceKindWriter for JList<T>
    where T: JceKindWriter
{
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(LIST, tag).format(b, 0);
        (self.len() as i32).to_bytes(b, 0);
        for v in self.iter() { v.to_bytes(b, 0) }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::super::{JceKindReader, JceKindWriter, JList, JString, LIST};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        vec![String::from("千橘"), String::from("雫霞")].to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![9, 0, 2, 6, 6, 229, 141, 131, 230, 169, 152, 6, 6, 233, 155, 171, 233, 156, 158]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JList::from_bytes(
                &mut Bytes::from(vec![0, 2, 6, 6, 229, 141, 131, 230, 169, 152, 6, 6, 233, 155, 171, 233, 156, 158]),
                LIST,
            ).unwrap() as JList<JString>,
            vec![String::from("千橘"), String::from("雫霞")],
        );
    }
}
