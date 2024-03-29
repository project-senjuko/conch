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

mod bytes2string;
mod get_sized;
mod put_str_with_16len;

/// 提供字节转换 utf8 字符串的方法
pub trait Bytes2String {
    /// 获取指定长度的 utf8 字符串
    ///
    /// # Safety
    ///
    /// 当指定长度 > remaining 时将 panic。
    fn get_string(&mut self, len: usize) -> String;
}

/// 提供获取特定长度字节的方法
pub trait GetSized {
    /// 获取特定长度字节
    ///
    /// # Safety
    ///
    /// 当指定长度 > remaining 时将 panic。
    fn get_sized(&mut self, len: usize) -> Self;
}

pub trait PutStrWith16Len {
    /// 写入头部为 16 位长度标识的字符串
    fn put_string_with_16len(&mut self, s: &str);
}
