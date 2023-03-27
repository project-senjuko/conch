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
        super::{MISC_BIT_MAP, SUB_SIG_MAP},
        TlvField,
    },
    bytes::{BufMut, BytesMut},
};

const VER: u8 = 0;

pub struct TlvT116 {}

impl TlvField for TlvT116 {
    fn tag() -> u16 {
        0x116
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(14);
        b.put_u8(VER);
        b.put_u32(MISC_BIT_MAP | 33554432); // UinDeviceToken = true, that const | 33554432
        b.put_u32(SUB_SIG_MAP);
        b.put_u8(1); // _sub_appid_list length (always 1) also named mDwSubAppidList
        b.put_u32(1600000226 /* msf_loginWithPicSt = true */); // _sub_appid_list item
    }
}
