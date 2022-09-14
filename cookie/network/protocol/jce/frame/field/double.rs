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

use super::{DOUBLE, HeadData, JceFieldErr, JceKind, JDouble};

impl JceKind for JDouble {
    type Type = JDouble;

    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(DOUBLE, tag).format(b, 8);
        b.put_f64(*self);
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> Result<Self::Type, JceFieldErr> { Ok(b.get_f64()) }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{DOUBLE, JceKind, JDouble};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        114.5141919810_f64.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![5, 64, 92, 160, 232, 133, 123, 144, 171]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JDouble::from_bytes(
                &mut Bytes::from(vec![64, 92, 160, 232, 133, 123, 144, 171]),
                DOUBLE,
            ).unwrap(),
            114.5141919810_f64,
        );
    }
}
