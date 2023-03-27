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
pub struct TlvT144 {
    pub t109: BytesMut,
    pub t52d: BytesMut,
    pub t124: BytesMut,
    pub t128: BytesMut,
    pub t16e: BytesMut,
}

impl TlvField for TlvT144 {
    fn tag(&self) -> u16 {
        0x144
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(2);
        b.put_u16(5); // tlv count
        b.extend(&self.t109);
        b.extend(&self.t52d);
        b.extend(&self.t124);
        b.extend(&self.t128);
        b.extend(&self.t16e);
    }
}
