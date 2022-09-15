////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, Bytes, BytesMut};

use super::{BOOL, JBool, JceFieldErr, JceKindReader, JceKindWriter, ZERO_TAG};

impl JceKindReader for JBool {
    type T = JBool;
    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<Self::T, JceFieldErr> {
        match r#type {
            BOOL => {
                b.advance(1);
                Ok(true)
            }
            ZERO_TAG => Ok(false),
            _ => Err(JceFieldErr { expectation: BOOL, result: r#type })
        }
    }
}

impl JceKindWriter for JBool {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        if *self { return 1_i8.to_bytes(b, tag); }
        0i8.to_bytes(b, tag);
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::super::{BOOL, JBool, JceKindReader, JceKindWriter, SHORT, ZERO_TAG};

    #[test]
    fn to_bytes_true() {
        let mut b = BytesMut::new();
        true.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![0, 1]);
    }

    #[test]
    fn to_bytes_false() {
        let mut b = BytesMut::new();
        false.to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![12]);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] // 适用该检查将导致语义含糊
    fn from_bytes_bool() {
        assert_eq!(
            JBool::from_bytes(&mut Bytes::from(vec![1]), BOOL).unwrap(),
            true,
        );
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn from_bytes_zero() {
        assert_eq!(
            JBool::from_bytes(&mut Bytes::from(vec![]), ZERO_TAG).unwrap(),
            false,
        );
    }

    #[test]
    #[should_panic]
    fn from_bytes_err() {
        JBool::from_bytes(&mut Bytes::from(vec![]), SHORT).unwrap();
    }
}
