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

//! # 机密信息
//!
//! 提供有关机密的初始化、读取、刷写等操作。
//! 存储于数据路径下 `secret` 文件中。

use {
    super::lifecycle::secret,
    anyhow::Result,
    rand::{distributions::Alphanumeric, thread_rng, Rng},
    rmp_serde::{Deserializer, Serializer},
    serde::{Deserialize, Serialize},
    std::fs::File,
    tracing::{debug, instrument},
    uuid::Uuid,
};

type B16 = [u8; 16];

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    #[serde(default)]
    pub account: u32,
    #[serde(default)]
    pub password: B16,

    #[serde(default = "rand_b16")]
    pub tgtgt: B16,

    #[serde(rename = "qimei", default = "rand_qimei")]
    pub rand_qimei: String,

    /// SSAID also named Android ID
    #[serde(rename = "ssaid", default = "rand_ssaid")]
    pub ssaid: String,
    #[serde(rename = "ssaid-md5", default)]
    pub ssaid_md5: B16,

    #[serde(rename = "mac-md5", default = "rand_b16")]
    pub mac_md5: B16,

    #[serde(rename = "boot-id", default = "rand_uuid")]
    pub boot_id: String,

    #[serde(default)]
    pub guid: B16,
}

impl Default for Secret {
    fn default() -> Self {
        Self {
            account: 0,
            password: Default::default(),
            tgtgt: rand_b16(),
            rand_qimei: rand_qimei(),
            ssaid: rand_ssaid(),
            ssaid_md5: Default::default(),
            mac_md5: rand_b16(),
            boot_id: rand_uuid(),
            guid: Default::default(),
        }
    }
}

/// 随机生成 16 字节
pub fn rand_b16() -> B16 {
    thread_rng().gen::<u128>().to_be_bytes()
}

/// 随机生成 UUID
pub fn rand_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Build rand ssaid (Android ID)
fn rand_ssaid() -> String {
    let mut rng = thread_rng();
    let s: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    s.to_ascii_lowercase()
}

/// 随机生成 QIMEI
pub fn rand_qimei() -> String {
    let mut rng = thread_rng();
    const CHARSET: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    (0..36)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())])
        .collect()
}

/// 提供机密信息方法
impl Secret {
    /// 读取机密信息
    #[instrument]
    pub async fn read() -> Self {
        let mut s = match secret().exists() {
            true => Secret::deserialize(&mut Deserializer::new(
                &File::open(secret()).expect("机密信息读取失败"),
            ))
                .expect("机密信息解析失败"),
            false => {
                debug!(dsc = "机密信息不存在，新建机密信息");
                Secret::default()
            }
        };

        s.compute();

        // 当向后兼容时，新增字段将持久化
        s.flash().await.expect("机密信息保存失败");
        debug!(dsc = "机密信息载入成功", sec = ?s);
        s
    }

    fn compute(&mut self) {
        self.ssaid_md5 = md5::compute(&self.ssaid).0;
        self.guid = md5::compute(self.ssaid.clone() + "02:00:00:00:00:00").0;
    }

    /// 保存机密信息
    pub async fn flash(&self) -> Result<()> {
        self.serialize(
            &mut Serializer::new(&mut File::create(secret()).expect("机密信息文件句柄获取失败"))
                .with_struct_map(),
        )?;
        Ok(())
    }
}
