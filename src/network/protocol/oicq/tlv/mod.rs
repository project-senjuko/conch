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

// 0x544 unknown 0x542

use bytes::{Buf, BufMut, BytesMut};

pub mod t1;
pub mod t8;
pub mod t18;
pub mod t100;
pub mod t106;
pub mod t107;
pub mod t109;
pub mod t116;
pub mod t124;
pub mod t128;
pub mod t141;
pub mod t142;
pub mod t144;
pub mod t145;
pub mod t147;
pub mod t154;
pub mod t16e;
pub mod t177;
pub mod t187;
pub mod t188;
pub mod t191;
pub mod t511;
pub mod t516;
pub mod t521;
pub mod t525;
pub mod t545;
pub mod t52d;
pub mod t548;

trait TlvField: Default {
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
        bytes::BytesMut,
        super::{t1::TlvT1, TlvField},
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
    fn to_bytes() { println!("{:?}", T1.to_bytes().to_vec()); }
}
