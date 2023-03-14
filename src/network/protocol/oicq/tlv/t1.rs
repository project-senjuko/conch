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
    time::OffsetDateTime,
    crate::runtime::Runtime,
    super::TlvField,
};

pub struct TlvT1 {
    pub ip_ver: u16,

    pub uin: u32,
}

impl Default for TlvT1 {
    fn default() -> Self {
        Self {
            ip_ver: 1,
            uin: Runtime::secret().account,
        }
    }
}

impl TlvField for TlvT1 {
    fn tag() -> u16 { 0x1 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(20);
        b.put_u16(self.ip_ver);
        b.put_u32(0x75757575); // rand, 75 = senju
        b.put_u32(self.uin);
        b.put_u32(OffsetDateTime::now_utc().unix_timestamp() as u32);
        b.put_bytes(0, 4);
        b.put_u16(0);
    }
}
