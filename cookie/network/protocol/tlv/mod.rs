////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BufMut, Bytes, BytesMut};

pub mod r#struct;

pub struct TlvField {
    pub command: u16,
    pub payload: Bytes,
}

impl TlvField {
    pub fn put_self(self, b: &mut BytesMut) {
        b.put_u16(self.command);
        b.put_u16(self.payload.remaining() as u16);
        b.put(self.payload);
    }
}

pub trait TlvTStruct {
    fn get_command() -> u16;

    fn to_tlv_payload(&self) -> Bytes;
    fn to_tlv_filed(&self) -> TlvField {
        TlvField { command: Self::get_command(), payload: self.to_tlv_payload() }
    }
}
