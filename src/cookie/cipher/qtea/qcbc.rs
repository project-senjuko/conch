////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::tea::TeaCipher;

pub struct QCBChaining {
    c: TeaCipher,
}

impl QCBChaining { pub fn new(c: TeaCipher) -> QCBChaining { QCBChaining { c } } }

impl QCBChaining {
    pub fn encrypt(&self, b: &mut Bytes) -> BytesMut {
        let mut bm = BytesMut::with_capacity(b.remaining());
        let (mut iv, mut av) = (0, 0);
        while b.remaining() > 0 {
            let before = b.get_u64() ^ iv; // pt ^ iv
            iv = self.c.encrypt(before) ^ av; // after ^ av = ct
            av = before;
            bm.put_u64(iv);
        }
        bm
    }

    pub fn decrypt(&self, b: &mut Bytes) -> BytesMut {
        let mut bm = BytesMut::with_capacity(b.remaining());
        let (mut iv, mut av) = (0, 0);
        while b.remaining() > 0 {
            let ct = b.get_u64();
            av = self.c.decrypt(ct ^ av); // before
            bm.put_u64(av ^ iv); // pt
            iv = ct;
        }
        bm
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::{QCBChaining, TeaCipher};

    const C: QCBChaining = QCBChaining { c: TeaCipher { key: [114, 514, 1919, 810] } };

    #[test]
    fn encrypt() {
        assert_eq!(
            C.encrypt(&mut Bytes::from(vec![2, 0, 2, 2, 2, 2, 0, 2, 5, 2, 0, 1, 3, 1, 4, 0])),
            vec![244, 123, 62, 197, 118, 127, 124, 229, 24, 107, 105, 26, 152, 90, 161, 238],
        );
    }

    #[test]
    fn decrypt() {
        assert_eq!(
            C.decrypt(&mut Bytes::from(vec![244, 123, 62, 197, 118, 127, 124, 229, 24, 107, 105, 26, 152, 90, 161, 238])),
            vec![2, 0, 2, 2, 2, 2, 0, 2, 5, 2, 0, 1, 3, 1, 4, 0],
        );
    }
}
