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
    crate::{
        common::upstream::{APP_SHORT_VER, APK_SIGNATURE_MD5},
        utils::bytes::PutStrWith16Len,
    },
};

pub struct TlvT147 {
    pub app_id: u32,
    pub apk_v: String,
    pub pkg_sig_from_apk_name: [u8; 16],
}

impl Default for TlvT147 {
    fn default() -> Self {
        Self {
            app_id: 16, // const need
            apk_v: APP_SHORT_VER.to_string(),
            pkg_sig_from_apk_name: APK_SIGNATURE_MD5,
        }
    }
}

impl TlvField for TlvT147 {
    fn tag() -> u16 { 0x147 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(0);
        b.put_u32(self.app_id);
        b.put_string_with_16len(&self.apk_v);
        b.put_u16(16); // length
        b.extend(self.pkg_sig_from_apk_name);
    }
}
