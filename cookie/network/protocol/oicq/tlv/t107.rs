////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{BufMut, BytesMut};

use super::TlvField;

#[derive(Default)]
struct TlvT107 {}

impl TlvField for TlvT107 {
    fn tag() -> u16 { 0x107 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(6);
        b.put_u16(0);
        b.put_u8(0);
        b.put_u16(0);
        b.put_u8(1);
    }
}
