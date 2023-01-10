////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{BufMut, Bytes, BytesMut};
use time::OffsetDateTime;

use super::TlvTStruct;

struct TlvT1 {
    uin: u32,
}

impl TlvTStruct for TlvT1 {
    fn get_command() -> u16 { 1 }

    fn to_tlv_payload(&self) -> Bytes {
        let mut b = BytesMut::with_capacity(20);
        b.put_u16(1);
        b.put_u32(757575); // 75 = senju
        b.put_u32(self.uin);
        b.put_u32(OffsetDateTime::now_utc().unix_timestamp() as u32);
        b.put_bytes(0, 4);
        b.put_u16(0);
        b.freeze()
    }
}
