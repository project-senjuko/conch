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
    crate::utils::bytes::PutStrWith16Len,
};

pub struct TlvT141 {
    pub version: u16,
    pub sim_operator_name: String,
    pub network_type: u16,
    pub apn_string: String,
}

impl Default for TlvT141 {
    fn default() -> Self {
        Self {
            version: 1,
            sim_operator_name: Default::default(),
            network_type: 2,
            apn_string: "wifi".to_string(), // const need
        }
    }
}

impl TlvField for TlvT141 {
    fn tag() -> u16 { 0x141 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(12);
        b.put_u16(self.version);
        b.put_string_with_16len(&self.sim_operator_name);
        b.put_u16(self.network_type);
        b.put_string_with_16len(&self.apn_string);
    }
}
