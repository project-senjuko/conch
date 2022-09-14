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

struct TlvT18 {
    uin: u32,
}

impl TlvTStruct for TlvT18 {
    fn get_command() -> u16 { 24 }

    fn to_tlv_payload(&self) -> Bytes {
        let mut b = BytesMut::with_capacity(22);
        b.put_u16(1);
        b.put_u32(1536);
        b.put_u32(16);
        b.put_u32(0);
        b.put_u32(self.uin);
        b.put_u16(0);
        b.put_u16(0);
        b.freeze()
    }
}
