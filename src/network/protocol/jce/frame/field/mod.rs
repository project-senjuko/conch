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

//! Jce 字段模块，
//! 定义 `Jce 原子类型` 和对应标签值、
//! 提供 `Jce 字段` 与 `Jce 字节流` 的序列化与反序列化特征、
//! 实现 `Jce 类型错误` 结构体。
//!
//! # `Kind` 与 `Struct`
//!
//! [`JceKindReader`] 和 [`JceKindWriter`] 基于 Jce 原子类型对其序列化与反序列化，支持的类型广泛普遍。
//! 当序列化或反序列化 `Jce 结构体` 类型时，调用 [`JceStructReader`] 或 [`JceStructWriter`] 特征中
//! 序列化或反序列化方法并封装或解封装 [`STRUCT_BEGIN`] 和 [`STRUCT_END`] 标记，并标有 tag。
//! 若直接通过 [`JceStructReader`] 或 [`JceStructWriter`] 生成，则不进行任何封装或解封装。生成的标准
//! `Jce 字节流` 可以直接作为最终产物使用，例：`RequestPack`。

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


/// Jce 类型读取器特征，
/// 提供 `Jce 字节流` 反序列化为 `Jce 类型` 的方法抽象。所有的 Jce 类型都必须实现本特征。
pub trait JceKindReader {
    /// Jce 类型，
    /// 指定 [`JceKindReader::from_bytes`] 方法返回参数的实体类型。
    type T;

    /// 将 `Jce 字节流` 反序列化为支持的 `Jce 类型`。
    /// `Jce 字节流` 中首个可被识别的内容块必须是 Jce 值的元数据。`r#type` 视反序列化目标类型的不同，可能会被忽略。
    fn from_bytes(b: &mut Bytes, r#type: u8) -> Result<Self::T, JceFieldErr>;
}

/// Jce 类型写入器特征，
/// 提供 `Jce 类型` 序列化为 `Jce 字节流` 的方法抽象。所有的 Jce 类型都必须实现本特征。
pub trait JceKindWriter {
    /// 将支持的 `Jce 类型` 序列化为 `Jce 字节流`。
    /// 序列化结果按实际大小(+1)预留容量后直接写入 `b` 缓冲区中。
    fn to_bytes(&self, b: &mut BytesMut, tag: u8);
}

/// Jce 结构体读取器特征。
/// 提供 `Jce 字节流` 反序列化为 `Jce 结构体` 的方法抽象。不是所有的 `Jce 结构体` 都实现了本特征，例如
/// 在只读不写的情况下 不实现本特征可以减小二进制内容大小。
pub trait JceStructReader {
    /// 将 `Jce 字节流` 反序列化为支持的 `Jce 结构体`。
    /// `Jce 字节流` 中首个可被识别的内容块必须是 Jce 头数据。
    fn s_from_bytes(&mut self, b: &mut Bytes) -> Result<(), JceFieldErr>;
}

/// Jce 结构体写入器特征。
/// 提供 `Jce 结构体` 序列化为 `Jce 字节流` 的方法抽象。不是所有的 `Jce 结构体` 都实现了本特征，例如
/// 在只写不读的情况下 不实现本特征可以减小二进制内容大小。
pub trait JceStructWriter {
    /// 将支持的 `Jce 结构体` 序列化为 `Jce 字节流`。
    /// 序列化结果按实际大小(+1)预留容量后直接写入 `b` 缓冲区中。
    fn s_to_bytes(&self, b: &mut BytesMut);
}

/// Jce 字段错误，
/// 实现错误特征、
/// 提供易读的错误消息。
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
