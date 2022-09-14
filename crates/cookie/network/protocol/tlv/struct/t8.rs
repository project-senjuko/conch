////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{BufMut, Bytes, BytesMut};

use super::TlvTStruct;

struct TlvT8 {}

impl TlvTStruct for TlvT8 {
    fn get_command() -> u16 { 8 }

    fn to_tlv_payload(&self) -> Bytes {
        let mut b = BytesMut::with_capacity(8);
        b.put_u16(0);
        b.put_u32(2052);
        b.put_u16(0);
        b.freeze()
    }
}
