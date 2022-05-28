use bytes::{BufMut, Bytes, BytesMut};

use crate::cookie::network::protocol::frame::taf::jce::field::{HeadData, JByte, JceStruct, JceType, JInt, JMap, JShort, JSList, JString, MAP, SIMPLE_LIST};

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
    fn s_to_bytes(&self) -> BytesMut {
        let mut b = self.version.to_bytes(1);
        b.put(self.packet_type.to_bytes(2));
        b.put(self.message_type.to_bytes(3));
        b.put(self.request_id.to_bytes(4));
        b.put(self.servant_name.to_bytes(5));
        b.put(self.func_name.to_bytes(6));
        b.put(self.buffer.to_bytes(7));
        b.put(self.timeout.to_bytes(8));
        b.put(self.context.to_bytes(9));
        b.put(self.status.to_bytes(10));
        b
    }

    fn s_from_bytes(mut self, b: &mut Bytes) -> JcePacket {
        {
            let h = HeadData::parse(b);
            self.version = JShort::from_bytes(b, h.r#type);
        }
        {
            let h = HeadData::parse(b);
            self.packet_type = JByte::from_bytes(b, h.r#type);
        }
        {
            let h = HeadData::parse(b);
            self.message_type = JInt::from_bytes(b, h.r#type);
        }
        {
            let h = HeadData::parse(b);
            self.request_id = JInt::from_bytes(b, h.r#type);
        }
        {
            let h = HeadData::parse(b);
            self.servant_name = String::from_bytes(b, h.r#type);
        }
        {
            let h = HeadData::parse(b);
            self.func_name = String::from_bytes(b, h.r#type);
        }
        {
            let _ = HeadData::parse(b);
            self.buffer = JSList::from_bytes(b, SIMPLE_LIST);
        }
        {
            let h = HeadData::parse(b);
            self.timeout = JInt::from_bytes(b, h.r#type);
        }
        {
            let _ = HeadData::parse(b);
            self.context = JMap::from_bytes(b, MAP);
        }
        {
            let _ = HeadData::parse(b);
            self.status = JMap::from_bytes(b, MAP);
        }
        self
    }

    fn init() -> JcePacket {
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
