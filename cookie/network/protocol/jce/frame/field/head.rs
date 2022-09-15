////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! Jce 头数据模块，
//! 定义 `Jce 头数据` 结构体、
//! 提供 `Jce 头数据` 与 `Jce 字节流` 的编解码、
//! 实现基于 `Jce 头数据` 的 `Jce 值` 跳过。

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, DOUBLE, FLOAT, INT, JceFieldErr, JceKindReader, JInt, LIST, LONG, MAP, SHORT, SIMPLE_LIST, STRING1, STRING4, STRUCT_BEGIN, STRUCT_END};

/// Jce 头数据，
/// 属于 `TT(L)V` 中 `TT` 部分、
/// 提供编码为 `Jce 字节流` 和解码为
/// `Jce 头数据` 的方法。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeadData {
    pub r#type: u8,
    pub tag: u8,
}

impl HeadData {
    /// 新建一个完整填充的 [`HeadData`] 结构体
    #[inline(always)]
    pub fn new(r#type: u8, tag: u8) -> Self { Self { r#type, tag } }

    /// 将 `Jce 字节流` 解码为 `Jce 头数据`
    pub fn parse(b: &mut Bytes) -> Self {
        let f = b.get_u8();
        let t = (f & 240) >> 4;
        Self { r#type: f & 15, tag: if t != 15 { t } else { b.get_u8() } }
    }

    /// 将 `Jce 头数据` 编码为 `Jce 字节流`，
    /// 编码结果直接写入 b: &mut [`BytesMut`] 中。
    pub fn format(&self, b: &mut BytesMut, additional: usize) {
        b.reserve(2 + additional);
        if self.tag <= 14 {
            b.put_u8(self.r#type | (self.tag << 4));
        } else {
            b.put_u8(self.r#type | 240);
            b.put_u8(self.tag);
        }
    }
}

impl HeadData {
    /// 将 `ttl4` 类型的 `Jce 字节流` 解码，
    /// 返回 `length` 数据。
    pub fn parse_ttl4(b: &mut Bytes) -> Result<usize, JceFieldErr> {
        let head = Self::parse(b);
        if head.tag != 0 { return Err(JceFieldErr { expectation: 255, result: 101 }); }
        Ok(JInt::from_bytes(b, head.r#type)? as usize)
    }

    /// 跳过 `value` 数据，
    /// 此方法通常用于从 `Jce 字节流` 中读取 [`HeadData`] 后
    /// 实际头数据与预期头数据不同时，暂时忽略此 `Jce 字段`。
    pub fn skip_value(&self, b: &mut Bytes) -> Result<(), JceFieldErr> {
        if self.r#type > 13 {
            return Err(JceFieldErr { expectation: 255, result: 100 });
        }

        let len = match self.r#type {
            BYTE => 1,
            SHORT => 2,
            INT => 4,
            LONG => 8,
            FLOAT => 4,
            DOUBLE => 8,
            STRING1 => b.get_u8() as usize,
            STRING4 => b.get_i32() as usize,
            MAP => {
                let len = Self::parse_ttl4(b)?;
                let mut i = 0;
                while i < len {
                    Self::parse(b).skip_value(b)?; // K
                    Self::parse(b).skip_value(b)?; // V
                    i += 1;
                }
                0
            }
            LIST => {
                let len = Self::parse_ttl4(b)?;
                let mut i = 0;
                while i < len {
                    Self::parse(b).skip_value(b)?;
                    i += 1;
                }
                0
            }
            STRUCT_BEGIN => {
                let mut h = Self::parse(b);
                while h.r#type != STRUCT_END {
                    h.skip_value(b)?;
                    h = Self::parse(b);
                }
                0
            }
            SIMPLE_LIST => 1 + Self::parse_ttl4(b)?, // 1: 0 type 0 tag head
            _ => 0, // STRUCT_END + ZERO_TAG
        };
        b.advance(len);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::HeadData;

    const A: HeadData = HeadData { r#type: 0, tag: 0 };
    const B: HeadData = HeadData { r#type: 1, tag: 0 };
    const C: HeadData = HeadData { r#type: 1, tag: 2 };
    const D: HeadData = HeadData { r#type: 2, tag: 8 };
    const E: HeadData = HeadData { r#type: 4, tag: 24 };

    #[test]
    fn parse() {
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![0])), A);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![1])), B);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![33])), C);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![130])), D);
        assert_eq!(HeadData::parse(&mut Bytes::from(vec![244, 24])), E);
    }

    #[test]
    fn format() {
        let mut b = BytesMut::new();
        A.format(&mut b, 0);
        assert_eq!(b.to_vec(), vec![0]);

        let mut b = BytesMut::new();
        B.format(&mut b, 0);
        assert_eq!(b.to_vec(), vec![1]);

        let mut b = BytesMut::new();
        C.format(&mut b, 0);
        assert_eq!(b.to_vec(), vec![33]);

        let mut b = BytesMut::new();
        D.format(&mut b, 0);
        assert_eq!(b.to_vec(), vec![130]);

        let mut b = BytesMut::new();
        E.format(&mut b, 0);
        assert_eq!(b.to_vec(), vec![244, 24]);
    }

    #[test]
    #[should_panic]
    fn skip_value_err() {
        HeadData { r#type: 114, tag: 0 }.skip_value(&mut Bytes::new()).unwrap();
    }
}
