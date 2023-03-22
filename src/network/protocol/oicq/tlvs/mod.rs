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

// 0x544 unknown 0x542

use bytes::{Buf, BufMut, BytesMut};

pub use {
    t1::*, t100::*, t106::*, t107::*, t109::*, t116::*, t124::*, t128::*, t141::*, t142::*,
    t144::*, t145::*, t147::*, t154::*, t16e::*, t177::*, t18::*, t187::*, t188::*, t191::*,
    t511::*, t516::*, t521::*, t525::*, t52d::*, t545::*, t548::*, t8::*,
};

mod t1;
mod t100;
mod t106;
mod t107;
mod t109;
mod t116;
mod t124;
mod t128;
mod t141;
mod t142;
mod t144;
mod t145;
mod t147;
mod t154;
mod t16e;
mod t177;
mod t18;
mod t187;
mod t188;
mod t191;
mod t511;
mod t516;
mod t521;
mod t525;
mod t52d;
mod t545;
mod t548;
mod t8;

trait TlvField {
    fn tag() -> u16;

    fn to_payload(&self, b: &mut BytesMut);

    fn put_bytes(&self, b: &mut BytesMut) {
        b.reserve(4);
        b.put_u16(Self::tag());
        let lenp = b.remaining();

        b.put_u16(0); // payload length
        self.to_payload(b);

        let l = (b.remaining() - lenp - 2) as u16;
        b[lenp..lenp + 2].swap_with_slice(&mut l.to_be_bytes()); // set payload length
    }

    fn to_bytes(&self) -> BytesMut {
        let mut b = BytesMut::with_capacity(4);
        self.put_bytes(&mut b);
        b
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{t1::TlvT1, TlvField},
        bytes::BytesMut,
    };

    const T1: TlvT1 = TlvT1 { ip_ver: 1, uin: 114514 };

    #[test]
    fn put_bytes() {
        let mut a = BytesMut::new();
        T1.put_bytes(&mut a);
        T1.put_bytes(&mut a);
        T1.put_bytes(&mut a);

        println!("{:?}", a.to_vec());
    }

    #[test]
    fn to_bytes() {
        println!("{:?}", T1.to_bytes().to_vec());
    }
}
