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

use super::{BYTE, HeadData, INT, JceFieldErr, JceKind, JLong, LONG, SHORT, ZERO_TAG};

impl JceKind for JLong {
    type Type = JLong;

    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self < 2147483648 && *self >= -2147483648 { return (*self as i32).to_bytes(b, tag); }
        HeadData::new(LONG, tag).format(b, 8);
        b.put_i64(*self);
    }

    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<Self::Type, JceFieldErr> {
        match r#type {
            BYTE => Ok(b.get_i8() as i64),
            SHORT => Ok(b.get_i16() as i64),
            INT => Ok(b.get_i32() as i64),
            LONG => Ok(b.get_i64()),
            ZERO_TAG => Ok(0),
            _ => Err(JceFieldErr { expectation: LONG, result: r#type }),
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{BYTE, INT, JceKind, JLong, LONG, SHORT, ZERO_TAG};
    use super::super::DOUBLE;

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        1145141919810_i64.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![3, 0, 0, 1, 10, 159, 199, 0, 66]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JLong::from_bytes(
                &mut Bytes::from(vec![0, 0, 1, 10, 159, 199, 0, 66]),
                LONG,
            ).unwrap(),
            1145141919810_i64,
        );
    }

    #[test]
    fn to_bytes_int() {
        let mut b = BytesMut::new();
        114514_i64.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![2, 0, 1, 191, 82]);
    }

    #[test]
    fn from_bytes_int() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![0, 1, 191, 82]), INT).unwrap(),
            114514_i64,
        );
    }

    #[test]
    fn from_bytes_short() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![7, 127]), SHORT).unwrap(),
            1919_i64,
        );
    }

    #[test]
    fn from_bytes_byte() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![114]), BYTE).unwrap(),
            114_i64,
        );
    }

    #[test]
    fn from_bytes_zero() {
        assert_eq!(
            JLong::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG).unwrap(),
            0_i64,
        );
    }

    #[test]
    #[should_panic]
    fn from_bytes_err() {
        JLong::from_bytes(&mut Bytes::from(vec![]), DOUBLE).unwrap();
    }
}
