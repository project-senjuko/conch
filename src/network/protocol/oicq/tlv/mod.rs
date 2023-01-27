////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{BufMut, BytesMut};

pub mod t1;
pub mod t8;
pub mod t18;
pub mod t100;
pub mod t106;
pub mod t107;
pub mod t116;
pub mod t142;

trait TlvField: Default {
    fn tag() -> u16;

    fn to_payload(&self, b: &mut BytesMut);

    fn to_bytes(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(4);

        b.put_u16(Self::tag());
        b.put_u16(0); // payload length
        self.to_payload(&mut b);

        let l = b.len() - 4;
        b[2..4].swap_with_slice(&mut l.to_be_bytes()); // set payload length

        b
    }
}
