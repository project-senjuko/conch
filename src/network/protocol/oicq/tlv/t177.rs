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
    crate::{
        utils::bytes::PutStrWith16Len,
        common::upstream::{BUILD_TIME, SDK_VERSION},
    },
};

#[derive(Default)]
struct TlvT177 {}

impl TlvField for TlvT177 {
    fn tag() -> u16 { 0x177 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(5);
        b.put_u8(1);
        b.put_u32(BUILD_TIME);
        b.put_string_with_16len(SDK_VERSION)
    }
}
