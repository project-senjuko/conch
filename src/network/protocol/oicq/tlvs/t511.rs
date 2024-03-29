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
pub struct TlvT511 {}

impl TlvField for TlvT511 {
    fn tag(&self) -> u16 {
        0x511
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.extend_from_slice(&[
            0x00, 0x0E, 0x01, 0x00, 0x0D, 0x6F, 0x66, 0x66, 0x69, 0x63, 0x65, 0x2E, 0x71, 0x71,
            0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0A, 0x71, 0x75, 0x6E, 0x2E, 0x71, 0x71, 0x2E,
            0x63, 0x6F, 0x6D, 0x01, 0x00, 0x11, 0x67, 0x61, 0x6D, 0x65, 0x63, 0x65, 0x6E, 0x74,
            0x65, 0x72, 0x2E, 0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0B, 0x64, 0x6F,
            0x63, 0x73, 0x2E, 0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0B, 0x6D, 0x61,
            0x69, 0x6C, 0x2E, 0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x09, 0x74, 0x69,
            0x2E, 0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0A, 0x76, 0x69, 0x70, 0x2E,
            0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0A, 0x74, 0x65, 0x6E, 0x70, 0x61,
            0x79, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0C, 0x71, 0x71, 0x77, 0x65, 0x62, 0x2E,
            0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0C, 0x71, 0x7A, 0x6F, 0x6E, 0x65,
            0x2E, 0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0A, 0x6D, 0x6D, 0x61, 0x2E,
            0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x0B, 0x67, 0x61, 0x6D, 0x65, 0x2E,
            0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00, 0x11, 0x6F, 0x70, 0x65, 0x6E, 0x6D,
            0x6F, 0x62, 0x69, 0x6C, 0x65, 0x2E, 0x71, 0x71, 0x2E, 0x63, 0x6F, 0x6D, 0x01, 0x00,
            0x0E, 0x63, 0x6F, 0x6E, 0x6E, 0x65, 0x63, 0x74, 0x2E, 0x71, 0x71, 0x2E, 0x63, 0x6F,
            0x6D,
        ]);
    }
}
