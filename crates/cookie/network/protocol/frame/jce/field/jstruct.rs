////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Bytes, BytesMut};

use super::{HeadData, JceFieldErr, JceKind, JceStruct, STRUCT_BEGIN, STRUCT_END};

impl<T> JceKind for T
    where T: JceStruct + Default
{
    type Type = T;

    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(STRUCT_BEGIN, tag).format(b, 0);
        self.s_to_bytes(b);
        HeadData::new(STRUCT_END, 0).format(b, 0);
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> Result<Self::Type, JceFieldErr> {
        let mut t = Self::Type::default();
        t.s_from_bytes(b)?;
        {
            let head = HeadData::parse(b);
            if head.tag != 0 || head.r#type != STRUCT_END {
                return Err(JceFieldErr { expectation: STRUCT_END, result: head.r#type });
            }
        }
        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::super::{HeadData, JceFieldErr, JceKind, JceStruct, STRING1, STRUCT_BEGIN};

    #[derive(Default, PartialEq, Debug)]
    struct Q {
        name: String,
    }

    impl JceStruct for Q {
        fn s_to_bytes(&self, b: &mut BytesMut) { self.name.to_bytes(b, 0); }

        fn s_from_bytes(&mut self, b: &mut Bytes) -> Result<(), JceFieldErr> {
            let _ = HeadData::parse(b);
            self.name = String::from_bytes(b, STRING1)?;
            Ok(())
        }
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
            Q::from_bytes(
                &mut Bytes::from(vec![6, 3, 229, 141, 131, 11]),
                STRUCT_BEGIN,
            ).unwrap(),
            Q { name: String::from("千") },
        );
    }
}
