use std::collections::HashMap;

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


#[derive(PartialEq, Debug)]
pub struct Field<T: JceType<T>> {
    pub key: HeadData,
    pub value: T,
}

/// 标准 Jce 类型必须具备的特征
pub trait JceType<T> {
    /// 将支持的类型格式化为字节流
    fn to_bytes(&self, tag: u8) -> BytesMut;
    /// 从字节流中解读支持的类型
    fn from_bytes(b: &mut Bytes, r#type: u8) -> T;
}

pub trait JceStruct<T> {
    /// 将支持的结构体格式化为字节流
    fn s_to_bytes(&self) -> BytesMut;
    /// 从字节流中解读支持的结构体
    fn s_from_bytes(mut self, b: &mut Bytes) -> T;
    /// 初始化结构体
    fn init() -> T;
}


const TYPE_ERR: &str = "Jce 类型不符";
