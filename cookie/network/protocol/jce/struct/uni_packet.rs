////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! Uni 数据包模块，
//! 实现 Jce 数据在 `Uni 数据包` 中的编解码。

use bytes::{Buf, BufMut, BytesMut};
use tracing::{instrument, trace};

use jce::field::{JceFieldErr, JceKindReader, JceKindWriter, JceStructReader, JceStructWriter, JInt, JMap, JSList, JString};

use crate::cipher::qtea::QTeaCipher;

use super::RequestPacket;

#[derive(Default)]
pub struct UniPacket {
    p: RequestPacket,
    data: JMap<JString, JSList>,
}

impl UniPacket {
    /// 新建一个基本的 `Uni 数据包(ver.3)`
    #[inline]
    pub fn new(rid: JInt, sn: &str, r#fn: &str) -> Self {
        Self {
            p: RequestPacket {
                version: 3,
                request_id: rid,
                servant_name: sn.to_string(),
                func_name: r#fn.to_string(),
                ..Default::default()
            },
            data: JMap::new(),
        }
    }

    /// 添加 `Jce 类型` 数据至本数据包
    pub fn put<T: JceKindWriter>(&mut self, n: &str, d: T) {
        let mut buf = BytesMut::new();
        d.to_bytes(&mut buf, 0);
        self.data.insert(n.to_string(), JSList::from(buf));
    }

    /// 将本数据包中所有数据编码为 UniPacket `Jce 字节流`
    pub fn encode(&mut self, b: &mut BytesMut) {
        let mut buf = BytesMut::new();
        self.data.to_bytes(&mut buf, 0);
        self.p.buffer = JSList::from(buf);

        let mut jp = BytesMut::new();
        self.p.s_to_bytes(&mut jp);

        let cap = jp.capacity();
        let mut up = BytesMut::with_capacity(cap + 4);
        up.put_i32(cap as i32);
        up.put(jp);
        b.put(up);
    }

    /// 将本数据包中所有数据编码为 UniPacket `Jce 字节流`，
    /// 并使用 [`QTeaCipher`] 加密。
    pub fn encode_with_tea(&mut self, key: [u32; 4]) -> BytesMut {
        let mut b = BytesMut::new();
        self.encode(&mut b);
        QTeaCipher::new(key).encrypt(b)
    }
}

impl UniPacket {
    /// 从加密的 UniPacket `Jce 字节流` 中解码为 `Uni 数据包(ver.3)`。
    /// 使用 [`QTeaCipher`] 解密。
    pub fn from(b: BytesMut, key: [u32; 4]) -> Result<Self, JceFieldErr> {
        let mut db = QTeaCipher::new(key).decrypt(b);
        db.get_i32(); // length

        let mut s = RequestPacket::default();
        s.s_from_bytes(&mut db.freeze())?;

        let i = s.buffer.get_u8();
        if i != 8 { // type == map && tag == 0
            return Err(JceFieldErr { expectation: 8, result: i });
        }

        Ok(Self { data: JMap::from_bytes(&mut s.buffer, 0)?, p: s })
    }

    /// 获取本数据包中 `Jce 类型`
    #[inline]
    pub fn get<T>(&mut self, n: &str) -> Result<T, JceFieldErr>
        where T: JceKindReader<T=T>
    {
        match self.data.get(n) {
            None => Err(JceFieldErr { expectation: 255, result: 201 }),
            Some(s) => T::from_bytes(&mut s.slice(1..), 0) // 固定字节 10: StructBegin Head
        }
    }
}
