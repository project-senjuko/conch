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

//! tgtgt qimei

use {
    bytes::BytesMut,
    super::TlvField,
    crate::runtime::Runtime,
};

#[derive(Default)]
struct TlvT545 {}

impl TlvField for TlvT545 {
    fn tag() -> u16 { 0x545 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(36);
        b.extend(Runtime::secret().rand_qimei.as_bytes());
    }
}
