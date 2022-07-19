////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{BufMut, Bytes, BytesMut};

use crate::{JceReader, JceWriter};
use crate::field::{JByte, JceStruct, JceType, JInt, JMap, JShort, JSList, JString};

pub struct JcePacketV3 {
    p: JcePacket,
    data: JMap<JString, JSList>,
}

impl JcePacketV3 {
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

    pub fn put<T: JceType<T>>(&mut self, n: &str, d: T) {
        let mut buf = BytesMut::new();
        d.to_bytes(&mut buf, 0);
        self.data.insert(n.to_string(), JSList::from(buf));
    }

    /// 编码为 UniPacket
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
}

/// ## 版本控制信息
/// struct-from | com.qq.taf.RequestPacket
/// qq-version | 8555
#[derive(Default)]
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

    fn s_from_bytes(&mut self, b: &mut Bytes) {
        let mut r = JceReader::with_tag(b, 1);
        self.version = r.get();
        self.packet_type = r.get();
        self.message_type = r.get();
        self.request_id = r.get();
        self.servant_name = r.get();
        self.func_name = r.get();
        self.buffer = r.get();
        self.timeout = r.get();
        self.context = r.get();
        self.status = r.get();
    }
}
