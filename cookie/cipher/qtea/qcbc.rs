////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BytesMut};

use super::TeaCipher;

pub struct QCBChaining {
    c: TeaCipher,
}

impl QCBChaining { pub fn new(c: TeaCipher) -> Self { Self { c } } }

impl QCBChaining {
    /// 加密
    pub fn encrypt(&self, mut b: BytesMut) -> BytesMut {
        let (r, mut i, mut iv, mut av) = (b.remaining(), 0, 0, 0);

        while r >= i + 8 {
            let before = Self::get_u64(&b, i) ^ iv; // pt ^ iv
            iv = self.c.encrypt(before) ^ av; // after ^ av = ct
            av = before;
            Self::put_u64(&mut b, i, iv);
            i += 8;
        }

        b
    }

    /// 解密
    pub fn decrypt(&self, mut b: BytesMut) -> BytesMut {
        let (r, mut i, mut iv, mut av) = (b.remaining(), 0, 0, 0);

        while r >= i + 8 {
            let ct = Self::get_u64(&b, i);
            av = self.c.decrypt(ct ^ av); // before
            Self::put_u64(&mut b, i, av ^ iv); // pt
            iv = ct;
            i += 8;
        }

        b
    }

    /// 获取一个大端序无符号的 64 位整数类型，
    /// 从索引 i 开始读取 8 个字节。
    #[inline]
    fn get_u64(b: &BytesMut, i: usize) -> u64 {
        let s: &[u8; 8] = b[i..i + 8].try_into().unwrap();
        u64::from_be_bytes(*s)
    }

    /// 写入一个大端序无符号类型的 64 位整数，
    /// 从索引 i 开始写入 8 个字节。
    #[inline]
    fn put_u64(b: &mut BytesMut, i: usize, n: u64) {
        b[i..i + 8].swap_with_slice(&mut u64::to_be_bytes(n));
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::{QCBChaining, TeaCipher};

    const C: QCBChaining = QCBChaining { c: TeaCipher { key: [114, 514, 1919, 810] } };

    #[test]
    fn encrypt() {
        assert_eq!(
            C.encrypt(BytesMut::from(&[2, 0, 2, 2, 2, 2, 0, 2, 5, 2, 0, 1, 3, 1, 4, 0][..])),
            vec![244, 123, 62, 197, 118, 127, 124, 229, 24, 107, 105, 26, 152, 90, 161, 238],
        );
    }

    #[test]
    fn decrypt() {
        assert_eq!(
            C.decrypt(BytesMut::from(&[244, 123, 62, 197, 118, 127, 124, 229, 24, 107, 105, 26, 152, 90, 161, 238][..])),
            vec![2, 0, 2, 2, 2, 2, 0, 2, 5, 2, 0, 1, 3, 1, 4, 0],
        );
    }
}
