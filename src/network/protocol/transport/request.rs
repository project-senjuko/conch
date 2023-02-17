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

use bytes::{Buf, BufMut, BytesMut};
use tracing::{instrument, warn};

use crate::cipher::qtea::QTeaCipher;
use crate::runtime::Runtime;
use crate::upstream::app_setting::{APP_COMMIT, APP_ID, APP_SHORT_VER};

use super::{EncryptionMethod, Flag, Packet, PacketBytes};

impl Packet {
    pub fn to_bytes(mut self) -> BytesMut {
        let mut b = BytesMut::new();

        self.encryption_method_fallback();

        b.put_u32(self.flag.to_u32());
        b.put_u8(self.encryption_method.to_u8());
        self.put_header(&mut b);
        b.put_u8(0);
        b.put_4string(&self.uin.to_string());

        let mut b2 = self.to_body();
        b2.put(self.buffer);

        match self.encryption_method {
            EncryptionMethod::UnEncrypted => {}
            EncryptionMethod::D2Encrypted =>
                b2 = QTeaCipher::new(Runtime::get_d2key()).encrypt(b2),
            EncryptionMethod::EmptyKeyEncrypted =>
                b2 = QTeaCipher::with_empty_key().encrypt(b2),
        }
        b.put(b2);

        b
    }

    #[instrument(skip(self))]
    #[inline]
    fn encryption_method_fallback(&mut self) {
        if self.encryption_method != EncryptionMethod::D2Encrypted
            ||
            Runtime::get_d2().remaining() != 0
        {
            return;
        }

        warn!(dsc = "加密模式降级");
        self.encryption_method = EncryptionMethod::EmptyKeyEncrypted
    }

    #[inline]
    fn put_header(&self, b: &mut BytesMut) {
        match self.flag {
            Flag::Login => {
                match self.encryption_method {
                    EncryptionMethod::UnEncrypted | EncryptionMethod::EmptyKeyEncrypted => {
                        b.put_u32(4); // 0 len
                    }
                    EncryptionMethod::D2Encrypted => {
                        let d = Runtime::get_d2();
                        b.put_u32((4 + d.remaining()) as u32);
                        b.put(d);
                    }
                }
            }
            Flag::Naive => b.put_u32(self.sequence_number),
        }
    }

    #[inline]
    fn to_body(&self) -> BytesMut {
        let mut b = BytesMut::new();
        b.put_u32(0); // length1

        if self.flag == Flag::Login {
            b.put_u32(self.sequence_number);
            b.put_u32(APP_ID);
            b.put_u32(APP_ID);
            b.put_slice(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);

            let t = Runtime::get_tgt();
            b.put_u32((4 + t.remaining()) as u32);
            b.put(t);
        }

        b.put_4string(&self.cmd);

        let m = Runtime::get_msg_cookie();
        b.put_u32((4 + m.remaining()) as u32);
        b.put(m);

        if self.flag == Flag::Login {
            b.put_4string(&Runtime::config().device.imei);
            b.put_u32(4); // ksid
            b.put_2string(&Self::build_ver());
        }

        b.put_u32(4); // 什么🐦 extraData
        let len = b.remaining() as u32;
        b[0..4].swap_with_slice(&mut len.to_be_bytes()); // head length

        b.put_u32((4 + self.buffer.remaining()) as u32); // value length
        b
    }

    #[inline]
    fn build_ver() -> String {
        "|".to_string() + &Runtime::config().device.imsi
            + "|A" + APP_SHORT_VER + "." + APP_COMMIT
    }
}
