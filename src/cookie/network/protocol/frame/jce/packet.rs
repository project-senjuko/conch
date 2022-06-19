////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Bytes, BytesMut};

use super::{JceReader, JceWriter};
use super::field::{JByte, JceStruct, JInt, JMap, JShort, JSList, JString};

/// ## 版本控制信息
/// struct-from | com.qq.taf.RequestPacket
/// qq-version | 8265
#[derive(Default)]
struct JcePacket {
    version: JShort,
    packet_type: JByte,
    message_type: JInt,
    request_id: JInt,
    servant_name: JString,
    func_name: JString,
    buffer: JSList,
    timeout: JInt,
    context: JMap<JString, JString>,
    status: JMap<JString, JString>,
}

impl JceStruct<JcePacket> for JcePacket {
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
