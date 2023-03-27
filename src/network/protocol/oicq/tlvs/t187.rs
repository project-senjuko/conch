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

use {super::TlvField, crate::runtime::Runtime, bytes::BytesMut};

pub struct TlvT187 {
    pub mac_md5: [u8; 16],
}

impl Default for TlvT187 {
    fn default() -> Self {
        Self {
            mac_md5: Runtime::secret().mac_md5,
        }
    }
}

impl TlvField for TlvT187 {
    fn tag(&self) -> u16 {
        0x187
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(16);
        b.extend_from_slice(&self.mac_md5);
    }
}
