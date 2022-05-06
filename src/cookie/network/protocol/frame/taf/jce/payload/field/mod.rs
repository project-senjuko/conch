mod head;
mod r#type;


use crate::cookie::network::protocol::frame::taf::jce::payload::field::head::HeadData;

use bytes::{Bytes, BytesMut};

#[derive(PartialEq, Debug)]
pub struct Field<T> {
    pub key: HeadData,
    pub value: T,
}

trait FieldBuild<T> {
    fn new(h: HeadData) -> Field<T>;
    fn with_value(h: HeadData, value: T) -> Field<T>;
    fn from_bytes(h: HeadData, b: &mut Bytes) -> Field<T>;
}

trait FieldReader { fn parse(&mut self, b: &mut Bytes); }

trait FieldWriter { fn format(&self) -> BytesMut; }
