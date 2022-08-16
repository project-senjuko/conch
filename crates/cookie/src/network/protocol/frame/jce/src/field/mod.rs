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
use std::error;
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


/// Jce 类型特征，
/// 提供 `Jce 类型` 与 `Jce 字节流` 之间序列化与反序列化、
/// 描述 Jce 类型应实现方法的签名。
///
/// 与 [`JceStruct`] 特征不同，本特征是 `JceStruct` 的基石，
/// 基于 Jce 原子类型对其序列化与反序列化，支持的类型广泛且普遍。
/// 序列化或反序列化 Jce 结构体类型时，先调用 [`JceStruct`] 特征中
/// 序列化或反序列化方法，将其字节流输出封装于或解封装
/// [`STRUCT_BEGIN`] 和 [`STRUCT_END`] 标记，并有 tag。
pub trait JceKind {
    /// Jce 类型
    type Type;

    /// 将支持的 `Jce 类型` 序列化为 `Jce 字节流`。
    /// 序列化结果直接写入 b: &mut [`BytesMut`] 中。
    fn to_bytes(&self, b: &mut BytesMut, tag: u8);

    /// 将 `Jce 字节流` 反序列化为支持的 `Jce 类型`。
    /// r#type 视反序列化目标类型不同，可能会被忽略。
    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<Self::Type, JceFieldErr>;
}

/// Jce 结构体特征。
/// 提供 `Jce 结构体` 与 `Jce 字节流` 之间序列化与反序列化、
/// 描述 Jce 结构体应实现方法的签名。
///
/// 与 [`JceKind`] 特征不同，本特征基于 `JceKind`。
/// 序列化或反序列化时，直接写入或读取 Jce 原子类型，
/// 而不进行任何封装或解封装。用于直接生成封装在
/// [`STRUCT_BEGIN`] 和 [`STRUCT_END`] 标记中的标准 Jce 字节流。
pub trait JceStruct {
    /// 将支持的 `Jce 结构体` 直接序列化为 `Jce 字节流`。
    /// 序列化结果直接写入 b: &mut [`BytesMut`] 中。
    fn s_to_bytes(&self, b: &mut BytesMut);

    /// 将 `Jce 字节流` 反序列化为支持的 `Jce 结构体`
    fn s_from_bytes(&mut self, b: &mut Bytes) -> Result<(), JceFieldErr>;
}

#[derive(Debug)]
pub struct JceFieldErr {
    pub expectation: u8,
    pub result: u8,
}

impl fmt::Display for JceFieldErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.expectation == 255 {
            match self.result {
                100 => write!(f, "无效的 Jce 类型"),
                101 => write!(f, "无效的 Jce 标签值"),
                102 => write!(f, "无效的 Jce utf-8 字符串"),
                200 => write!(f, "缺少必须的 Jce 字段"),
                201 => write!(f, "指定的 JcePacket Key 不存在"),
                _ => write!(f, "无效的 Jce 错误码")
            }
        } else if self.expectation == self.result {
            write!(f, "Jce 字段预期标签与实际不符")
        } else {
            write!(f, "Jce 字段预期类型与实际不符 预期: {} 实际: {}", self.expectation, self.result)
        }
    }
}

impl error::Error for JceFieldErr {}
