////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub account: u32,
    pub password: B16,

    pub tgtgt: B16,

    #[serde(rename = "android-id-md5")]  pub android_id_md5: B16,
    pub guid: B16,
}

impl Default for Secret {
    fn default() -> Self {
        Self {
            account: 0,
            password: Default::default(),
            tgtgt: rand_b16(),
            android_id_md5: rand_b16(),
            guid: rand_b16(),
        }
    }
}

/// 随机生成 16 字节
pub fn rand_b16() -> B16 { thread_rng().gen::<u128>().to_be_bytes() }

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
                let s = Secret::default();
                s.flash().await.expect("机密信息保存失败");
                s
            }
        };

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
