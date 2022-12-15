////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use anyhow::{bail, Result};
use bytes::{Buf, BytesMut};
use tracing::error;

use crate::util::bytes::{Bytes2String, GetSized};

mod request;
mod respond;

/// 包
#[derive(Debug, Default)]
pub struct Packet {
    flag: Flag,
    encryption_method: EncryptionMethod,
    sequence_number: u32,
    uin: u64,
    cmd: String,
    buffer: BytesMut,
}

#[derive(Debug, Default)]
pub enum Flag {
    Login,
    #[default] Naive,
}

/// 加密模式
#[derive(Debug, Default)]
pub enum EncryptionMethod {
    /// 未加密
    UnEncrypted,

    /// D2 加密
    D2Encrypted,

    /// 空密钥加密
    #[default] EmptyKeyEncrypted,
}

impl Flag {
    fn to_u32(&self) -> u32 {
        match self {
            Flag::Login => 0x0A,
            Flag::Naive => 0x0B,
        }
    }

    fn try_from_u32(o: u32) -> Result<Self> {
        match o {
            0x0A => Ok(Self::Login),
            0x0B => Ok(Self::Naive),
            _ => {
                const DSC: &str = "识别 flag 失败";
                error!(dsc = DSC, flag = o);
                bail!(DSC);
            }
        }
    }
}

impl EncryptionMethod {
    fn to_u8(&self) -> u8 {
        match self {
            EncryptionMethod::UnEncrypted => 0,
            EncryptionMethod::D2Encrypted => 1,
            EncryptionMethod::EmptyKeyEncrypted => 2,
        }
    }

    fn try_from_u8(o: u8) -> Result<Self> {
        match o {
            0 => Ok(EncryptionMethod::UnEncrypted),
            1 => Ok(EncryptionMethod::D2Encrypted),
            2 => Ok(EncryptionMethod::EmptyKeyEncrypted),
            _ => {
                const DSC: &str = "识别加密模式失败";
                error!(dsc = DSC, emn = o);
                bail!(DSC);
            }
        }
    }
}

trait PacketBytes {
    /// 获取 4 字节标识长度的 utf8 字符串
    fn get_4string(&mut self) -> String;

    fn get_4sized(&mut self) -> Self;
}

impl PacketBytes for BytesMut {
    fn get_4string(&mut self) -> String {
        let l = self.get_u32() as usize - 4;
        self.get_string(l)
    }

    fn get_4sized(&mut self) -> Self {
        let l = self.get_u32() as usize - 4;
        self.get_sized(l)
    }
}
