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
    time::OffsetDateTime,
    crate::{cipher::qtea::QTeaCipher, runtime::Runtime, common::upstream::APP_ID},
    super::TlvField,
};

pub struct TlvT106 {
    pub tgtgt_ver: u16,
    pub sso_ver: u32,

    pub uin: u64,
    pub password: [u8; 16],
}

impl Default for TlvT106 {
    fn default() -> Self {
        Self {
            tgtgt_ver: 4,
            sso_ver: 19,
            uin: Runtime::secret().account as u64,
            password: Runtime::secret().password,
        }
    }
}

impl TlvField for TlvT106 {
    fn tag() -> u16 { 0x106 }

    fn to_payload(&self, b: &mut BytesMut) {
        let mut bi = BytesMut::with_capacity(110);

        bi.put_u16(self.tgtgt_ver);
        bi.put_u32(0x75757575); // rand
        bi.put_u32(self.sso_ver);
        bi.put_u32(16); // appid
        bi.put_u32(0);
        bi.put_u64(self.uin);

        let t = OffsetDateTime::now_utc().unix_timestamp() as u32;
        bi.put_u32(t);
        bi.put_u32(0);
        bi.put_u8(1);
        bi.put_slice(&self.password);

        let mut tgtgto = BytesMut::with_capacity(28);
        tgtgto.put_u64(self.uin);
        tgtgto.put_u32(t);
        tgtgto.put_slice(&self.password);

        bi.put_slice(&md5::compute(tgtgto).0); // tgtgt
        bi.put_u32(0);
        bi.put_u8(1);
        bi.put_u128(0x75757575757575757575757575757575); // guid
        bi.put_u32(APP_ID);
        bi.put_u32(1);

        let us = self.uin.to_string();
        bi.put_u16(us.len() as u16);
        bi.put_slice(us.as_ref());
        bi.put_u16(0);

        let mut key = BytesMut::with_capacity(12);
        key.put_slice(&self.password);
        key.put_u32(0);
        key.put_u32(self.uin as u32);

        let p = QTeaCipher::with_16key(md5::compute(key).0).encrypt(bi);
        b.extend_from_slice(p.as_ref());
    }
}