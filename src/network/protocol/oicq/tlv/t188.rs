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
    bytes::BytesMut,
    crate::runtime::Runtime,
    super::TlvField,
};

pub struct TlvT188 {
    pub android_id_md5: [u8; 16],
}

impl Default for TlvT188 {
    fn default() -> Self {
        Self {
            android_id_md5: Runtime::secret().android_id_md5,
        }
    }
}

impl TlvField for TlvT188 {
    fn tag() -> u16 { 0x188 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(16);
        b.extend_from_slice(&self.android_id_md5);
    }
}
