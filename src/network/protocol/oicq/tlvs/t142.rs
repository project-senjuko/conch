////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

use {
    bytes::{BufMut, BytesMut},
    crate::common::upstream::APK_NAME,
    super::TlvField,
};

#[derive(Default)]
pub struct TlvT142 {
    pub version: u16, // 0
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
