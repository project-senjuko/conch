use bytes::{Bytes, BytesMut};

use crate::cookie::network::protocol::frame::jce::{JceReader, JceWriter};

use super::field::{JByte, JceStruct, JInt, JMap, JShort, JSList, JString};

/// ## 版本控制信息
/// struct-from | com.qq.taf.RequestPacket
/// qq-version | 7975
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
        let mut w = JceWriter::with_tag(1);
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
        w.to_bytes(b);
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

    fn new() -> JcePacket {
        JcePacket {
            version: 0,
            packet_type: 0,
            message_type: 0,
            request_id: 0,
            servant_name: JString::new(),
            func_name: JString::new(),
            buffer: JSList::new(),
            timeout: 0,
            context: JMap::new(),
            status: JMap::new(),
        }
    }
}
