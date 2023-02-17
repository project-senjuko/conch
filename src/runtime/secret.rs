////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

//! # 机密信息
//!
//! 提供有关机密的初始化、读取、刷写等操作。
//! 存储于数据路径下 `secret` 文件中。

use {
    anyhow::Result,
    rand::{Rng, thread_rng},
    rmp_serde::{Deserializer, Serializer},
    serde::{Deserialize, Serialize},
    std::fs::File,
    super::lifecycle::secret,
    tracing::{debug, instrument},
};

type B16 = [u8; 16];

/// 机密结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    /// 帐号
    #[serde(default)] pub account: u32,
    /// 密码
    #[serde(default)] pub password: B16,

    /// TGTGT
    #[serde(default = "rand_b16")] pub tgtgt: B16,

    /// Android ID MD5
    #[serde(rename = "android-id-md5", default = "rand_b16")] pub android_id_md5: B16,
    #[serde(rename = "mac-md5", default = "rand_b16")] pub mac_md5: B16,
    /// GUID
    #[serde(default = "rand_b16")] pub guid: B16,
}

impl Default for Secret {
    fn default() -> Self {
        Self {
            account: 0,
            password: Default::default(),
            tgtgt: rand_b16(),
            android_id_md5: rand_b16(),
            mac_md5: rand_b16(),
            guid: rand_b16(),
        }
    }
}

/// 随机生成 16 字节
pub fn rand_b16() -> B16 { thread_rng().gen::<u128>().to_be_bytes() }

/// 提供机密信息方法
impl Secret {
    /// 读取机密信息
    #[instrument]
    pub async fn read() -> Self {
        let s = match secret().exists() {
            true => Secret::deserialize(&mut Deserializer::new(
                &File::open(secret()).expect("机密信息读取失败"),
            )).expect("机密信息解析失败"),
            false => {
                debug!(dsc = "机密信息不存在，新建机密信息");
                Secret::default()
            }
        };

        // 当向后兼容时，新增字段将持久化
        s.flash().await.expect("机密信息保存失败");
        debug!(dsc = "机密信息载入成功", sec = ?s);
        s
    }

    /// 保存机密信息
    pub async fn flash(&self) -> Result<()> {
        self.serialize(&mut Serializer::new(
            &mut File::create(secret()).expect("机密信息文件句柄获取失败"),
        ).with_struct_map())?;
        Ok(())
    }
}
