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
        super::{APP_CLIENT_VERSION, APP_ID as OICQ_APP_ID, DW_MAIN_SIG_MAP},
        TlvField,
    },
    crate::common::upstream::APP_ID,
    bytes::{BufMut, BytesMut},
};

const DB_BUF_VER: u16 = 1;
const SSO_VER: u32 = 19;

#[derive(Default)]
pub struct TlvT100 {}

impl TlvField for TlvT100 {
    fn tag() -> u16 {
        0x100
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(22);
        b.put_u16(DB_BUF_VER);
        b.put_u32(SSO_VER);
        b.put_u32(OICQ_APP_ID);
        b.put_u32(APP_ID);
        b.put_u32(APP_CLIENT_VERSION);
        b.put_u32(DW_MAIN_SIG_MAP | 192); // const | 192 at GetStWithPasswd
    }
}
