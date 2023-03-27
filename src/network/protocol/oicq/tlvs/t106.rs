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
    super::{
        super::{APP_CLIENT_VERSION, APP_ID as OICQ_APP_ID, IP_ADDR},
        TlvField,
    },
    crate::{
        cipher::qtea::QTeaCipher, common::upstream::APP_ID as SUB_APP_ID,
        utils::bytes::PutStrWith16Len,
    },
    bytes::{BufMut, BytesMut},
    rand::{thread_rng, Rng},
};

const TGTGT_VER: u16 = 4;
const SS0_VER: u32 = 19;

pub struct TlvT106 {
    pub uin: u64,
    pub request_init_time: u32,
    pub password: [u8; 16],
    pub tgtgt_key: [u8; 16],
    pub guid: [u8; 16],
}

impl TlvField for TlvT106 {
    fn tag(&self) -> u16 {
        0x106
    }

    fn to_payload(&self, b: &mut BytesMut) {
        let mut bi = BytesMut::with_capacity(110);

        bi.put_u16(TGTGT_VER);
        bi.put_u32(thread_rng().gen()); // rand
        bi.put_u32(SS0_VER);
        bi.put_u32(OICQ_APP_ID);
        bi.put_u32(APP_CLIENT_VERSION);
        bi.put_u64(self.uin);
        bi.put_u32(self.request_init_time);
        bi.extend_from_slice(&IP_ADDR);
        bi.put_u8(1); // pinned at previous stack
        bi.extend_from_slice(&self.password);
        bi.extend_from_slice(&self.tgtgt_key);
        bi.put_u32(0); // pinned
        bi.put_u8(1); // means guid is available, 0 if not
        bi.extend_from_slice(&self.guid);
        bi.put_u32(SUB_APP_ID);
        bi.put_u32(1); // when "is sms login" = false, NOTE: it's integer instead of bool(0/1)
        bi.put_string_with_16len(&self.uin.to_string());
        bi.put_u16(0); // stupid + 2 length with empty, wtf qq is

        let mut key = BytesMut::with_capacity(24);
        key.extend_from_slice(&self.password);
        key.put_u64(self.uin);

        b.extend_from_slice(
            QTeaCipher::with_16key(md5::compute(key).0)
                .encrypt(bi)
                .as_ref(),
        );
    }
}
