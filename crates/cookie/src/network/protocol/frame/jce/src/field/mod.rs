////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::fmt;

use bytes::{Bytes, BytesMut};

pub use self::head::HeadData;

mod head;

mod byte;
mod bool;
mod short;
mod int;
mod long;
mod float;
mod double;
mod string;
mod map;
mod list;
mod jstruct;
mod jslist;


pub type JByte = i8;
pub type JBool = bool;
pub type JShort = i16;
pub type JInt = i32;
pub type JLong = i64;
pub type JFloat = f32;
pub type JDouble = f64;
pub type JString = String;
pub type JMap<T, U> = HashMap<T, U>;
pub type JList<T> = Vec<T>;
pub type JSList = Bytes;

pub const BYTE: u8 = 0;
pub const BOOL: u8 = 0;
pub const SHORT: u8 = 1;
pub const INT: u8 = 2;
pub const LONG: u8 = 3;
pub const FLOAT: u8 = 4;
pub const DOUBLE: u8 = 5;
pub const STRING1: u8 = 6;
pub const STRING4: u8 = 7;
pub const MAP: u8 = 8;
pub const LIST: u8 = 9;
pub const STRUCT_BEGIN: u8 = 10;
pub const STRUCT_END: u8 = 11;
pub const ZERO_TAG: u8 = 12;
pub const SIMPLE_LIST: u8 = 13;


#[derive(PartialEq, Debug)]
pub struct Field<T: JceType<T>> {
    pub key: HeadData,
    pub value: T,
}

/// 标准 Jce 类型必须具备的特征
pub trait JceType<T> {
    /// 将支持的类型格式化为字节流
    fn to_bytes(&self, b: &mut BytesMut, tag: u8);
    /// 从字节流中解读支持的类型
    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<T, JceFieldErr>;
}

pub trait JceStruct {
    /// 将支持的结构体格式化为字节流
    fn s_to_bytes(&self, b: &mut BytesMut);
    /// 从字节流中解读支持的结构体
    fn s_from_bytes(&mut self, b: &mut Bytes) -> Result<(), JceFieldErr>;
}

/// Jce 字段错误，大部分在预期类型与实际不符时生成，
/// 但也可以在期望标签与实际不符 或 无法正常识别 Jce 字段时 生成。
/// 若预期类型与实际相同则是标签不符；
/// 若期望类型是 255，则实际类型是错误代码。
/// 100：无效的 Jce 类型
/// 101：无效的标签值
/// 102：无效的 utf-8 字符串
/// 200：缺少必须的字段
/// 201：指定的 Key 不存在
#[derive(Debug)]
pub struct JceFieldErr {
    pub expectation: u8,
    pub result: u8,
}

impl fmt::Display for JceFieldErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Jce 字段预期与实际不符 预期类型: {} 实际: {}", self.expectation, self.result)
    }
}
