////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BufMut, BytesMut};

use qcbc::QCBChaining;
pub use tea::K;
use tea::TeaCipher;

mod qcbc;
mod tea;

pub struct QTeaCipher {
    c: QCBChaining,
}

impl QTeaCipher {
    #[inline]
    pub fn new(key: K) -> Self { Self { c: QCBChaining::new(TeaCipher::new(key)) } }

    #[inline]
    pub fn with_empty_key() -> Self { Self::new(<K as Default>::default()) }
}

impl QTeaCipher {
    pub fn encrypt(&self, b: BytesMut) -> BytesMut {
        let len = b.remaining();
        let fixed = 10 + len;
        let mut fill = fixed % 8;
        if fill != 0 { fill = 8 - fill; }
        let head = 2 + fill;

        let mut bm = BytesMut::with_capacity(fixed + fill);
        bm.put_u8(fill as u8 | 248);
        bm.put_bytes(75, head); // 75 = senju
        bm.put_slice(&b);
        bm.put_bytes(0, 7);

        self.c.encrypt(bm)
    }

    pub fn decrypt(&self, b: BytesMut) -> BytesMut {
        let mut bm = self.c.decrypt(b);
        let len = bm.remaining();
        let head = ((bm.get_u8() & 7) + 3) as usize;
        bm.split_off(head - 1).split_to(len - head - 7)
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::QTeaCipher;

    #[test]
    fn encrypt() {
        assert_eq!(
            QTeaCipher::new([75, 7565, 6576, 76]). // senjunakasumi
                encrypt(BytesMut::from(&[2, 0, 2, 2][..])).to_vec(),
            vec![159, 224, 47, 148, 141, 93, 112, 191, 185, 78, 235, 150, 76, 140, 182, 252],
        );
    }

    #[test]
    fn decrypt() {
        assert_eq!(
            QTeaCipher::new([75, 7565, 6576, 76]).
                decrypt(BytesMut::from(&[159, 224, 47, 148, 141, 93, 112, 191, 185, 78, 235, 150, 76, 140, 182, 252][..])),
            vec![2, 0, 2, 2],
        );
    }
}
