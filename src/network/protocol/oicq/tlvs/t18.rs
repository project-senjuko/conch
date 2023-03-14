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
    crate::runtime::Runtime,
    super::TlvField,
};

pub struct TlvT18 {
    pub ping_version: u16,
    pub sso_version: u32,

    pub uin: u32,
}

impl Default for TlvT18 {
    fn default() -> Self {
        Self {
            ping_version: 1,
            sso_version: 1536,
            uin: Runtime::secret().account,
        }
    }
}

impl TlvField for TlvT18 {
    fn tag() -> u16 { 0x18 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(22);
        b.put_u16(self.ping_version);
        b.put_u32(self.sso_version);
        b.put_u32(16); // appid
        b.put_u32(0);
        b.put_u32(self.uin);
        b.put_u16(0);
        b.put_u16(0);
    }
}
