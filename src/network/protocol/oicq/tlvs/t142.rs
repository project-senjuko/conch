////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.moe>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
//   More information at https://github.com/project-senjuko/conch.                                 /
////////////////////////////////////////////////////////////////////////////////////////////////////

use {
    super::TlvField,
    crate::common::upstream::APK_NAME,
    bytes::{BufMut, BytesMut},
};

const VERSION: u16 = 0;

#[derive(Default)]
pub struct TlvT142 {}

impl TlvField for TlvT142 {
    fn tag() -> u16 {
        0x142
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(24);
        b.put_u16(VERSION);
        b.put_u16(APK_NAME.len() as u16);
        b.extend_from_slice(APK_NAME.as_ref());
    }
}
