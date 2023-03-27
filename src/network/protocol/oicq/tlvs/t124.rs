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
    super::TlvField,
    crate::utils::bytes::PutStrWith16Len,
    bytes::{BufMut, BytesMut},
};

pub struct TlvT124 {
    pub os_type: String,
    pub os_version: String,
    pub network_type: u16,
    pub sim_operator_name: String,
    pub wtf: String,
    pub apn_string: String,
}

impl Default for TlvT124 {
    fn default() -> Self {
        Self {
            os_type: String::from("android"),
            os_version: String::from("14"), // need const
            network_type: 2,
            sim_operator_name: Default::default(),
            wtf: Default::default(),
            apn_string: String::from("wifi"),
        }
    }
}

impl TlvField for TlvT124 {
    fn tag(&self) -> u16 {
        0x124
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(25);
        b.put_string_with_16len(&self.os_type);
        b.put_string_with_16len(&self.os_version);
        b.put_u16(self.network_type);
        b.put_string_with_16len(&self.sim_operator_name);
        b.put_string_with_16len(&self.wtf);
        b.put_string_with_16len(&self.apn_string);
    }
}
