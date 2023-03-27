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

use {super::TlvField, bytes::BytesMut};

#[derive(Default)]
pub struct TlvT525 {}

impl TlvField for TlvT525 {
    fn tag(&self) -> u16 {
        0x525
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(8);
        b.extend([
            0x00, 0x01, // tlv length: 1
            0x05, 0x36, // tlv tag: TGTGT 0x536
            0x00, 0x02, // tlv length: 2
            0x01, 0x00, // LoginExtraData: i wanna know wtf mean [help wanted]
        ]);
    }
}
