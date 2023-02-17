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

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{FLOAT, HeadData, JceFieldErr, JceKindReader, JceKindWriter, JFloat};

impl JceKindReader for JFloat {
    type T = JFloat;
    fn from_bytes(b: &mut Bytes, _: u8) -> Result<Self::T, JceFieldErr> { Ok(b.get_f32()) }
}

impl JceKindWriter for JFloat {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(FLOAT, tag).format(b, 4);
        b.put_f32(*self);
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::super::{FLOAT, JceKindReader, JceKindWriter, JFloat};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        11.4_f32.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![4, 65, 54, 102, 102]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JFloat::from_bytes(&mut Bytes::from(vec![65, 54, 102, 102]), FLOAT).unwrap(),
            11.4_f32,
        );
    }
}
