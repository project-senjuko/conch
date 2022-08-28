////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

const DELTA: u32 = 0x9E37_79B9;

type V = [u32; 2];
type K = [u32; 4];

pub struct TeaCipher {
    pub key: K,
}

impl TeaCipher { pub fn new(key: K) -> TeaCipher { TeaCipher { key } } }

impl TeaCipher {
    pub fn encrypt(&self, v: u64) -> u64 { from(self._encrypt(to(v))) }

    pub fn decrypt(&self, v: u64) -> u64 { from(self._decrypt(to(v))) }

    fn _encrypt(&self, [mut v0, mut v1]: V) -> V {
        let [k0, k1, k2, k3] = self.key;
        let (mut sum, mut i) = (0u32, 0);
        while i < 16 {
            sum = sum.wrapping_add(DELTA);
            v0 = v0.wrapping_add((v1 << 4).wrapping_add(k0) ^ v1.wrapping_add(sum) ^ (v1 >> 5).wrapping_add(k1));
            v1 = v1.wrapping_add((v0 << 4).wrapping_add(k2) ^ v0.wrapping_add(sum) ^ (v0 >> 5).wrapping_add(k3));
            i += 1;
        }
        [v0, v1]
    }

    fn _decrypt(&self, [mut v0, mut v1]: V) -> V {
        let [k0, k1, k2, k3] = self.key;
        let (mut sum, mut i) = (0xE377_9B90_u32, 0);
        while i < 16 {
            v1 = v1.wrapping_sub((v0 << 4).wrapping_add(k2) ^ v0.wrapping_add(sum) ^ (v0 >> 5).wrapping_add(k3));
            v0 = v0.wrapping_sub((v1 << 4).wrapping_add(k0) ^ v1.wrapping_add(sum) ^ (v1 >> 5).wrapping_add(k1));
            sum = sum.wrapping_sub(DELTA);
            i += 1;
        }
        [v0, v1]
    }
}

fn to(n: u64) -> V { [(n >> 32) as u32, n as u32] }

fn from([v0, v1]: V) -> u64 { (v0 as u64) << 32 | v1 as u64 }

#[cfg(test)]
mod tests {
    use super::TeaCipher;

    const C: TeaCipher = TeaCipher { key: [114, 514, 1919, 810] };

    #[test]
    fn encrypt() { assert_eq!(C.encrypt(8848), 9146115744461577458); }

    #[test]
    fn decrypt() { assert_eq!(C.decrypt(9146115744461577458), 8848); }
}
