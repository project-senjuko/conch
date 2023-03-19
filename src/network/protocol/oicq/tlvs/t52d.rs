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
    crate::{network::protocol::protobuf::oicq::DeviceReport, runtime::Runtime},
    bytes::BytesMut,
    prost::Message,
};

#[derive(Default)]
pub struct TlvT52d {}

impl TlvField for TlvT52d {
    fn tag() -> u16 {
        0x52d
    }

    fn to_payload(&self, b: &mut BytesMut) {
        let secret = Runtime::secret();
        let ver = Vec::from("221024.007");

        let device_report = DeviceReport {
            bootloader: Vec::from("unknown"),
            version: Default::default(),
            codename: Vec::from("REL"),
            incremental: ver.clone(),
            fingerprint: Vec::from(Runtime::config().device.fingerprint.clone()),
            boot_id: Vec::from(secret.boot_id.clone()),
            android_id: Vec::from(secret.ssaid.clone()),
            baseband: Default::default(),
            inner_ver: ver,
        };
        b.reserve(device_report.encoded_len());
        device_report.encode(b).unwrap();
    }
}
