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

//! QTea 密码学实现，
//! 提供加解密功能。

use bytes::{Buf, BufMut, BytesMut};

/// Tea key schedule constant
const DELTA: u32 = 0x9E37_79B9;

type V = [u32; 2];
pub type K = [u32; 4];

/// QTeaCipher 结构体
pub struct QTeaCipher {
    /// 密钥
    key: K,
}

/// [`QTeaCipher`] 实例创建方法
impl QTeaCipher {
    /// 新建一个 [`QTeaCipher`] 实例，
    /// 使用标准密码。
    #[inline]
    pub fn new(key: K) -> Self { Self { key } }

    /// 新建使用空密钥的 [`QTeaCipher`] 实例
    #[inline]
    pub fn with_empty_key() -> Self { Self::new(Default::default()) }

    /// 新建使用 16 长度密钥的 [`QTeaCipher`] 实例
    pub fn with_16key(k: [u8; 16]) -> Self {
        Self::new([
            u32::from_be_bytes([k[0], k[1], k[2], k[3]]),
            u32::from_be_bytes([k[4], k[5], k[6], k[7]]),
            u32::from_be_bytes([k[8], k[9], k[10], k[11]]),
            u32::from_be_bytes([k[12], k[13], k[14], k[15]]),
        ])
    }
}

/// QTea 密码学实现
impl QTeaCipher {
    /// QTea 加密
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

        self.qcbc_encrypt(bm)
    }

    /// QTea 解密
    pub fn decrypt(&self, b: BytesMut) -> BytesMut {
        let mut bm = self.qcbc_decrypt(b);
        let len = bm.remaining();
        let head = ((bm.get_u8() & 7) + 3) as usize;
        bm.split_off(head - 1).split_to(len - head - 7)
    }

    /// QCBC 加密
    pub fn qcbc_encrypt(&self, mut b: BytesMut) -> BytesMut {
        let (r, mut i, mut iv, mut av) = (b.remaining(), 0, 0, 0);

        while r >= i + 8 {
            let before = Self::get_u64(&b, i) ^ iv; // pt ^ iv
            iv = self.tea_encrypt(before) ^ av; // after ^ av = ct
            av = before;
            Self::put_u64(&mut b, i, iv);
            i += 8;
        }

        b
    }

    /// QCBC 解密
    pub fn qcbc_decrypt(&self, mut b: BytesMut) -> BytesMut {
        let (r, mut i, mut iv, mut av) = (b.remaining(), 0, 0, 0);

        while r >= i + 8 {
            let ct = Self::get_u64(&b, i);
            av = self.tea_decrypt(ct ^ av); // before
            Self::put_u64(&mut b, i, av ^ iv); // pt
            iv = ct;
            i += 8;
        }

        b
    }

    /// Tea 加密，
    /// 此函数包装原始加密算法，以支持 u64 加密运算。
    #[inline]
    pub fn tea_encrypt(&self, v: u64) -> u64 {
        Self::v_to_u64(Self::vanilla_encrypt(self.key, Self::u64_to_v(v)))
    }

    /// Tea 解密，
    /// 此函数包装原始解密算法，以支持 u64 解密运算。
    #[inline]
    pub fn tea_decrypt(&self, v: u64) -> u64 {
        Self::v_to_u64(Self::vanilla_decrypt(self.key, Self::u64_to_v(v)))
    }

    /// Thanks https://en.wikipedia.org/wiki/Tiny_Encryption_Algorithm#Reference_code.

    /// Tea 核心加密
    #[inline]
    fn vanilla_encrypt([k0, k1, k2, k3]: K, [mut v0, mut v1]: V) -> V {
        let (mut sum, mut i) = (0u32, 0);
        while i < 16 {
            sum = sum.wrapping_add(DELTA);
            v0 = v0.wrapping_add((v1 << 4).wrapping_add(k0) ^ v1.wrapping_add(sum) ^ (v1 >> 5).wrapping_add(k1));
            v1 = v1.wrapping_add((v0 << 4).wrapping_add(k2) ^ v0.wrapping_add(sum) ^ (v0 >> 5).wrapping_add(k3));
            i += 1;
        }
        [v0, v1]
    }

    /// Tea 核心解密
    #[inline]
    fn vanilla_decrypt([k0, k1, k2, k3]: K, [mut v0, mut v1]: V) -> V {
        let (mut sum, mut i) = (0xE377_9B90_u32, 0); // sum: QTea 只进行 16 轮
        while i < 16 {
            v1 = v1.wrapping_sub((v0 << 4).wrapping_add(k2) ^ v0.wrapping_add(sum) ^ (v0 >> 5).wrapping_add(k3));
            v0 = v0.wrapping_sub((v1 << 4).wrapping_add(k0) ^ v1.wrapping_add(sum) ^ (v1 >> 5).wrapping_add(k1));
            sum = sum.wrapping_sub(DELTA);
            i += 1;
        }
        [v0, v1]
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

    /// u64 转换为 [`QTeaCipher::vanilla_encrypt`] 和 [`QTeaCipher::vanilla_decrypt`] 支持的入参。
    #[inline]
    fn u64_to_v(n: u64) -> V { [(n >> 32) as u32, n as u32] }

    /// [`QTeaCipher::vanilla_encrypt`] 和 [`QTeaCipher::vanilla_decrypt`] 的运算结果转换为 u64。
    #[inline]
    fn v_to_u64([v0, v1]: V) -> u64 { (v0 as u64) << 32 | v1 as u64 }
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
