////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! Jce 包模块，
//! 定义 `Jce 请求包` 结构体、
//! 实现 Jce 数据在 `Jce 请求包` 中的编解码。

use bytes::{Buf, BufMut, Bytes, BytesMut};

use qtea::QTeaCipher;

use crate::{JceReader, JceWriter};
use crate::field::{JByte, JceFieldErr, JceKind, JceStruct, JInt, JMap, JShort, JSList, JString};

#[derive(Default)]
pub struct JcePacketV3 {
    p: JcePacket,
    data: JMap<JString, JSList>,
}

impl JcePacketV3 {
    /// 新建一个基本的 `Jce 请求包(ver.3)`
    #[inline(always)]
    pub fn new(rid: JInt, sn: &str, r#fn: &str) -> JcePacketV3 {
        JcePacketV3 {
            p: JcePacket {
                version: 3,
                request_id: rid,
                servant_name: sn.to_string(),
                func_name: r#fn.to_string(),
                ..Default::default()
            },
            data: JMap::new(),
        }
    }

    /// 添加 `Jce 类型` 数据至本请求包
    pub fn put<T: JceKind>(&mut self, n: &str, d: T) {
        let mut buf = BytesMut::new();
        d.to_bytes(&mut buf, 0);
        self.data.insert(n.to_string(), JSList::from(buf));
    }

    /// 将本请求包中所有数据编码为 UniPacket `Jce 字节流`
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

    /// 将本请求包中所有数据编码为 UniPacket `Jce 字节流`，
    /// 并使用 [`QTeaCipher`] 加密。
    pub fn encode_with_tea(&mut self, key: [u32; 4]) -> BytesMut {
        let mut b = BytesMut::new();
        self.encode(&mut b);
        QTeaCipher::new(key).encrypt(&b.freeze())
    }
}

impl JcePacketV3 {
    /// 从加密的 UniPacket `Jce 字节流` 中解码为 `Jce 请求包(ver.3)`。
    /// 使用 [`QTeaCipher`] 解密。
    pub fn from(b: &mut Bytes, key: [u32; 4]) -> Result<JcePacketV3, JceFieldErr> {
        let mut db = QTeaCipher::new(key).decrypt(b);
        db.get_i32(); // length

        let mut s = JcePacket::default();
        s.s_from_bytes(&mut db.freeze())?;

        let i = s.buffer.get_u8();
        if i != 8 { // type == map && tag == 0
            return Err(JceFieldErr { expectation: 8, result: i });
        }

        Ok(JcePacketV3 { data: JMap::from_bytes(&mut s.buffer, 0)?, p: s })
    }

    /// 获取本请求包中 `Jce 类型`
    #[inline(always)]
    pub fn get<T>(&mut self, n: &str) -> Result<T, JceFieldErr>
        where T: JceKind<Type=T>
    {
        match self.data.get(n) {
            None => Err(JceFieldErr { expectation: 255, result: 201 }),
            Some(s) => T::from_bytes(&mut s.slice(1..), 0) // 固定字节 10: StructBegin Head
        }
    }
}

/// 源 | com.qq.taf.RequestPacket
#[derive(Default, Debug)]
pub struct JcePacket {
    pub version: JShort,
    pub packet_type: JByte,
    pub message_type: JInt,
    pub request_id: JInt,
    pub servant_name: JString,
    pub func_name: JString,
    pub buffer: JSList,
    pub timeout: JInt,
    pub context: JMap<JString, JString>,
    pub status: JMap<JString, JString>,
}

impl JceStruct for JcePacket {
    fn s_to_bytes(&self, b: &mut BytesMut) {
        let mut w = JceWriter::new(1);
        w.put(&self.version);
        w.put(&self.packet_type);
        w.put(&self.message_type);
        w.put(&self.request_id);
        w.put(&self.servant_name);
        w.put(&self.func_name);
        w.put(&self.buffer);
        w.put(&self.timeout);
        w.put(&self.context);
        w.put(&self.status);
        w.flash(b);
    }

    fn s_from_bytes(&mut self, b: &mut Bytes) -> Result<(), JceFieldErr> {
        let mut r = JceReader::with_tag(b, 1);
        self.version = r.get()?;
        self.packet_type = r.get()?;
        self.message_type = r.get()?;
        self.request_id = r.get()?;
        self.servant_name = r.get()?;
        self.func_name = r.get()?;
        self.buffer = r.get()?;
        self.timeout = r.get()?;
        self.context = r.get()?;
        self.status = r.get()?;
        Ok(())
    }
}
