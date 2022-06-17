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

use super::{BYTE, HeadData, JByte, JceType, TYPE_ERR, ZERO_TAG};

impl JceType<JByte> for JByte {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self == 0 { return HeadData::new(ZERO_TAG, tag).format(b, 0); }
        HeadData::new(BYTE, tag).format(b, 1);
        b.put_i8(*self);
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JByte {
        match r#type {
            BYTE => b.get_i8(),
            ZERO_TAG => 0,
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{BYTE, JByte, JceType, ZERO_TAG};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        114_i8.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![0, 114]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(JByte::from_bytes(&mut Bytes::from(vec![114]), BYTE), 114_i8);
    }

    #[test]
    fn to_bytes_zero() {
        let mut b = BytesMut::new();
        0_i8.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![12]);
    }

    #[test]
    fn from_bytes_zero() {
        assert_eq!(JByte::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG), 0_i8);
    }
}
