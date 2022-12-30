////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, HeadData, JceFieldErr, JceKindReader, JceKindWriter, JShort, SHORT, ZERO_TAG};

impl JceKindReader for JShort {
    type T = JShort;
    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<Self::T, JceFieldErr> {
        match r#type {
            BYTE => Ok(b.get_i8() as i16),
            SHORT => Ok(b.get_i16()),
            ZERO_TAG => Ok(0),
            _ => Err(JceFieldErr { expectation: SHORT, result: r#type }),
        }
    }
}

impl JceKindWriter for JShort {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self < 128 && *self >= -128 { return (*self as i8).to_bytes(b, tag); }
        HeadData::new(SHORT, tag).format(b, 2);
        b.put_i16(*self);
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::super::{BYTE, INT, JceKindReader, JceKindWriter, JShort, SHORT, ZERO_TAG};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        1919_i16.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![1, 7, 127]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JShort::from_bytes(&mut Bytes::from(vec![7, 127]), SHORT).unwrap(),
            1919_i16,
        );
    }

    #[test]
    fn to_bytes_byte() {
        let mut b = BytesMut::new();
        114_i16.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![0, 114]);
    }

    #[test]
    fn from_bytes_byte() {
        assert_eq!(
            JShort::from_bytes(&mut Bytes::from(vec![114]), BYTE).unwrap(),
            114_i16,
        );
    }

    #[test]
    fn from_bytes_zero() {
        assert_eq!(
            JShort::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG).unwrap(),
            0_i16,
        );
    }

    #[test]
    #[should_panic]
    fn from_bytes_err() {
        JShort::from_bytes(&mut Bytes::from(vec![]), INT).unwrap();
    }
}
