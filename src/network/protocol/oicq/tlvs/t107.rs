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
    bytes::{BufMut, BytesMut},
};

#[derive(Default)]
pub struct TlvT107 {}

impl TlvField for TlvT107 {
    fn tag() -> u16 {
        0x107
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(6);
        b.put_u16(0); // PicType
        b.put_u8(0); // pinned at previous stack
        b.put_u16(0); // pinned at previous stack
        b.put_u8(1); // pinned at previous stack
    }
}
