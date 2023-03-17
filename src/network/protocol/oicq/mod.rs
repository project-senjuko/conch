////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.moe>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
//   More information at https://github.com/project-senjuko/conch.                                 /
////////////////////////////////////////////////////////////////////////////////////////////////////

use {
    anyhow::{bail, Result},
    tracing::{error, instrument},
};

pub mod request;
pub mod tlvs;

/// OICQ Message Struct
pub struct Message {
    uin: u32,
    cmd: u16,
    encryption_method: EncryptionMethod,
}

/// # Encryption Method
#[derive(Debug, Default, Eq, PartialEq)]
pub enum EncryptionMethod {
    /// ECDH 加密
    #[default]
    Ecdh,

    /// ST 加密
    St,
}

/// # Encryption Method Impl
impl EncryptionMethod {
    /// # Convert Method To U8
    fn to_u8(&self) -> u8 {
        match self {
            EncryptionMethod::Ecdh => 135, // 有个 7 不知道什么东西
            EncryptionMethod::St => 69,
        }
    }

    /// # Try Convert U8 To Method
    #[instrument]
    fn try_from_u8(o: u8) -> Result<Self> {
        match o {
            135 => Ok(EncryptionMethod::Ecdh),
            69 => Ok(EncryptionMethod::St),
            7 => {
                error!(dsc = "意外的加密模式");
                Ok(EncryptionMethod::Ecdh)
            }
            _ => {
                const DSC: &str = "识别加密模式失败";
                error!(dsc = DSC, emn = o);
                bail!(DSC);
            }
        }
    }
}
