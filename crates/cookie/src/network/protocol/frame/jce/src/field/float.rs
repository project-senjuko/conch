////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{FLOAT, HeadData, JceFieldErr, JceKind, JFloat};

impl JceKind for JFloat {
    type Type = JFloat;

    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(FLOAT, tag).format(b, 4);
        b.put_f32(*self);
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> Result<Self::Type, JceFieldErr> { Ok(b.get_f32()) }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{FLOAT, JceKind, JFloat};

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
