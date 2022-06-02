use bytes::{Bytes, BytesMut};

use super::field::{HeadData, JByte, JceStruct, JceType, JInt, JMap, JShort, JSList, JString, MAP, SIMPLE_LIST};

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
        self.version.to_bytes(b, 1);
        self.packet_type.to_bytes(b, 2);
        self.message_type.to_bytes(b, 3);
        self.request_id.to_bytes(b, 4);
        self.servant_name.to_bytes(b, 5);
        self.func_name.to_bytes(b, 6);
        self.buffer.to_bytes(b, 7);
        self.timeout.to_bytes(b, 8);
        self.context.to_bytes(b, 9);
        self.status.to_bytes(b, 10);
    }

    fn s_from_bytes(&mut self, b: &mut Bytes) {
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
