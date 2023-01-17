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

use crate::upstream::app_setting::APK_NAME;

use super::TlvField;

#[derive(Default)]
struct TlvT142 {
    version: u16, // 0
}

impl TlvField for TlvT142 {
    fn tag() -> u16 { 0x142 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(24);
        b.put_u16(self.version);
        b.put_u16(APK_NAME.len() as u16);
        b.put_slice(APK_NAME.as_ref());
    }
}
