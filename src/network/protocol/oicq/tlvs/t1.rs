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
    super::{super::IP_ADDR, TlvField},
    bytes::{BufMut, BytesMut},
    rand::{thread_rng, Rng},
};

const IP_VER: u16 = 1;

pub struct TlvT1 {
    pub uin: u32,
    pub server_cur_time: u32,
}

impl TlvField for TlvT1 {
    fn tag(&self) -> u16 {
        0x1
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(20);
        b.put_u16(IP_VER);
        b.put_u32(thread_rng().gen());
        b.put_u32(self.uin);
        b.put_u32(self.server_cur_time);
        b.extend_from_slice(&IP_ADDR);
        b.put_u16(0); // pinned
    }
}
