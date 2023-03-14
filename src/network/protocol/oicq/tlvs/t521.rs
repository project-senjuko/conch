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
pub struct TlvT521 {
    pub product_type: u32, // default 0
}

impl TlvField for TlvT521 {
    fn tag() -> u16 { 0x521 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(6);
        b.put_u32(self.product_type);
        b.put_u16(0);
    }
}
