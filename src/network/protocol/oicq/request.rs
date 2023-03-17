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

//! OICQ Request Message

use {
    super::{EncryptionMethod, Message},
    bytes::{Buf, BufMut, BytesMut},
    rand::{thread_rng, Rng},
};

/// Message Request Impl
impl Message {
    /// # Build Request Message
    ///
    /// ## From
    ///
    /// oicq.wlogin_sdk.request.oicq_request - public final void a(int, int, long, int, int, int, int, int)
    pub fn to_bytes(self) -> BytesMut {
        let mut b = BytesMut::new();

        b.put_u8(2); // start flag

        b.put_u16(0); // reserve length
        b.put_u16(8001);
        b.put_u16(self.cmd);
        b.put_u16(1);
        b.put_u32(self.uin);
        b.put_u8(3);
        b.put_u8(self.encryption_method.to_u8());
        b.put_u8(0);
        b.put_u32(2);
        b.put_u32(0);
        b.put_u32(0);
        b.put(self.to_body());

        b.put_u8(3); // end flag

        let len = b.remaining() as u16;
        b[1..3].swap_with_slice(&mut len.to_be_bytes()); // set length

        b
    }

    /// # Build Request Payload
    #[inline]
    fn to_body(&self) -> BytesMut {
        let mut b = BytesMut::new();

        match self.encryption_method {
            EncryptionMethod::Ecdh => {
                b.put_u8(2);
                b.put_u8(1);
                b.put_u128(thread_rng().gen());
                b.put_u16(305); // cipher suite ver
                b.put_u16(0); // key ver todo
                b.put_u16(0); // key length todo
                              //b.put(); // key todo
                              //b.put(); // message with encrypt todo
            }
            EncryptionMethod::St => {
                unreachable!("EncryptionMethod:st is not supported yet");
            }
        }

        b
    }
}
