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
    serde::{Deserialize, Serialize},
    super::lifecycle::secret,
    tokio::fs::read_to_string,
    tracing::{error, instrument},
};

type B16 = [u8; 16];

#[derive(Debug, Serialize, Deserialize, /*临时*/ Default)]
pub struct Secret {
    #[serde(default)] pub account: u32,
    #[serde(default)] pub password: B16,

    #[serde(default = "default::rand_b16")] pub tgtgt: B16,

    #[serde(default = "default::rand_b16")] pub android_id_md5: B16,
    #[serde(default = "default::rand_b16")] pub guid: B16,
}

mod default {
    use {
        rand::{Rng, thread_rng},
        super::B16,
    };

    pub fn rand_b16() -> B16 { thread_rng().gen::<u128>().to_be_bytes() }
}

impl Secret {
    #[instrument]
    pub async fn read() -> Self {
        let b = read_to_string(secret()).await;
        if b.is_err() {
            error!(dsc = "读取失败", err = %b.as_ref().unwrap_err());
        }

        // TODO 接入 msgpack
        Self::default()
    }
}
