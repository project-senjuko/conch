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
    bytes::{BufMut, BytesMut},
    crate::common::upstream::APP_ID,
    super::TlvField,
};

pub struct TlvT100 {
    pub db_buf_ver: u16,
    pub sso_ver: u32,

    pub app_id: u32,
}

impl Default for TlvT100 {
    fn default() -> Self {
        Self {
            db_buf_ver: 1,
            sso_ver: 19,
            app_id: APP_ID,
        }
    }
}

impl TlvField for TlvT100 {
    fn tag() -> u16 { 0x100 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(22);
        b.put_u16(self.db_buf_ver);
        b.put_u32(self.sso_ver);
        b.put_u32(16); // appid
        b.put_u32(self.app_id);
        b.put_u32(0); // app client version
        b.put_u32(34869472); // main sigmap = (mDwMainSigMap = 34869344) | 192
    }
}
