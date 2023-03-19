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

pub struct TlvT109 {
    pub android_id_md5: [u8; 16], // need
}

impl Default for TlvT109 {
    fn default() -> Self {
        Self {
            android_id_md5: md5::compute(&Runtime::secret().ssaid).0,
        }
    }
}

impl TlvField for TlvT109 {
    fn tag() -> u16 {
        0x109
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(16);
        b.extend_from_slice(&self.android_id_md5);
    }
}
