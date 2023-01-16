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
struct TlvT116 {
    ver: u8, // 0
}

impl TlvField for TlvT116 {
    fn tag() -> u16 { 0x116 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(0);
        b.put_u8(self.ver);
        b.put_u32(184024956); // mMiscBitmap
        b.put_u32(66560); // mSubSigMap
        b.put_u8(1); // _sub_appid_list length
        b.put_u32(1600000226); // _sub_appid_list item
    }
}
