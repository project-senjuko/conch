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
    std::sync::atomic::Ordering,
    super::TlvField,
    crate::runtime::sequence::MSF_SSO_SEQ,
};

#[derive(Default)]
struct TlvT154 {}

impl TlvField for TlvT154 {
    fn tag() -> u16 { 0x154 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(4);
        b.put_u32(MSF_SSO_SEQ.load(Ordering::Relaxed));
    }
}
