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

use super::{BYTE, HeadData, INT, JceFieldErr, JceKind, JInt, SHORT, ZERO_TAG};

impl JceKind for JInt {
    type Type = JInt;

    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self < 32768 && *self >= -32768 { return (*self as i16).to_bytes(b, tag); }
        HeadData::new(INT, tag).format(b, 4);
        b.put_i32(*self);
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<Self::Type, JceFieldErr> {
        match r#type {
            BYTE => Ok(b.get_i8() as i32),
            SHORT => Ok(b.get_i16() as i32),
            INT => Ok(b.get_i32()),
            ZERO_TAG => Ok(0),
            _ => Err(JceFieldErr { expectation: INT, result: r#type }),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{INT, JceKind, JInt, SHORT, ZERO_TAG};
    use super::super::LONG;

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        114514_i32.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![2, 0, 1, 191, 82]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JInt::from_bytes(&mut Bytes::from(vec![0, 1, 191, 82]), INT).unwrap(),
            114514_i32,
        );
    }

    #[test]
    fn to_bytes_short() {
        let mut b = BytesMut::new();
        1919_i32.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![1, 7, 127]);
    }

    #[test]
    fn from_bytes_short() {
        assert_eq!(
            JInt::from_bytes(&mut Bytes::from(vec![7, 127]), SHORT).unwrap(),
            1919_i32,
        );
    }

    #[test]
    fn from_bytes_zero() {
        assert_eq!(
            JInt::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG).unwrap(),
            0_i32,
        );
    }

    #[test]
    #[should_panic]
    fn from_bytes_err() {
        JInt::from_bytes(&mut Bytes::from(vec![]), LONG).unwrap();
    }
}
