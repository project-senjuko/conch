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

//! canWebVerify

use {
    super::TlvField,
    bytes::{BufMut, BytesMut},
};

#[derive(Default)]
pub struct TlvT191 {}

impl TlvField for TlvT191 {
    fn tag(&self) -> u16 {
        0x191
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(1);
        b.put_u8(130);
    }
}
