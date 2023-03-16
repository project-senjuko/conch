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
    super::TlvField,
};

#[derive(Default)]
pub struct TlvT8 {}

impl TlvField for TlvT8 {
    fn tag() -> u16 { 0x8 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(8);
        b.put_u16(0);
        b.put_u32(2052);
        b.put_u16(0);
    }
}