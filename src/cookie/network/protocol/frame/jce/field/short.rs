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

use super::{BYTE, HeadData, JceType, JShort, SHORT, TYPE_ERR, ZERO_TAG};

impl JceType<JShort> for JShort {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self < 128 && *self >= -128 { return (*self as i8).to_bytes(b, tag); }
        HeadData::new(SHORT, tag).format(b, 2);
        b.put_i16(*self);
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> JShort {
        match r#type {
            BYTE => b.get_i8() as i16,
            SHORT => b.get_i16(),
            ZERO_TAG => 0,
            _ => panic!("{}", TYPE_ERR),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{BYTE, JceType, JShort, SHORT};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        1919_i16.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![1, 7, 127]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(JShort::from_bytes(&mut Bytes::from(vec![7, 127]), SHORT), 1919_i16);
    }

    #[test]
    fn to_bytes_byte() {
        let mut b = BytesMut::new();
        114_i16.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![0, 114]);
    }

    #[test]
    fn from_bytes_byte() {
        assert_eq!(JShort::from_bytes(&mut Bytes::from(vec![114]), BYTE), 114_i16);
    }
}
