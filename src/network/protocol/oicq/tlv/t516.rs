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
    bytes::{BytesMut, BufMut},
    super::TlvField,
};

#[derive(Default)]
struct TlvT516 {}

impl TlvField for TlvT516 {
    fn tag() -> u16 { 0x516 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(4);
        b.put_u32(0);
    }
}
