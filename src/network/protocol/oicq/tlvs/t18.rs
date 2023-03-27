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
    super::{
        super::{APP_CLIENT_VERSION, APP_ID},
        TlvField,
    },
    bytes::{BufMut, BytesMut},
};

const PING_VERSION: u16 = 1;
const SSO_VERSION: u32 = 1536;

pub struct TlvT18 {
    pub uin: u32,
}

impl TlvField for TlvT18 {
    fn tag() -> u16 {
        0x18
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(22);
        b.put_u16(PING_VERSION);
        b.put_u32(SSO_VERSION);
        b.put_u32(APP_ID);
        b.put_u32(APP_CLIENT_VERSION);
        b.put_u32(self.uin);
        b.put_u16(0); // pinned at previous stack
        b.put_u16(0); // pinned
    }
}
