////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

mod bytes2string;
mod get_sized;

/// 提供获取特定长度字节的方法
pub trait GetSized {
    /// 获取特定长度字节
    ///
    /// # Safety
    ///
    /// 当指定长度 > remaining 时将 panic。
    fn get_sized(&mut self, len: usize) -> Self;
}

/// 提供字节转换 utf8 字符串的方法
pub trait Bytes2String {
    /// 获取指定长度的 utf8 字符串
    ///
    /// # Safety
    ///
    /// 当指定长度 > remaining 时将 panic。
    fn get_string(&mut self, len: usize) -> String;
}
