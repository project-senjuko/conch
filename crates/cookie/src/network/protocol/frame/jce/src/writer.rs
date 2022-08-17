////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! Jce 字节流写入器，
//! 提供 `Jce 类型` 编码为 `Jce 字节流`。

use bytes::BytesMut;

use crate::field::JceKind;

/// Jce 字节流写入器
pub struct JceWriter<'a> {
    b: &'a mut BytesMut,
    tag: u8,
}

impl<'a> JceWriter<'a> {
    /// 新建一个完整填充的 `Jce 字节流写入器`
    #[inline(always)]
    pub fn new(b: &'a mut BytesMut, tag: u8) -> Self { Self { b, tag } }
}

impl JceWriter<'_> {
    /// 设置 tag 指针数值
    #[inline(always)]
    pub fn set_tag(&mut self, t: u8) { self.tag = t; }

    /// 添加 `Jce 类型` 数据至本写入器
    #[inline(always)]
    pub fn put<T: JceKind>(&mut self, t: &T) {
        t.to_bytes(self.b, self.tag);
        self.set_tag(self.tag + 1);
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::JceWriter;

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        let mut w = JceWriter::new(&mut b, 1);
        w.put(&1);
        w.put(&String::from("千橘橘"));
        assert_eq!(b.to_vec(), vec![16, 1, 38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152]);
    }
}
