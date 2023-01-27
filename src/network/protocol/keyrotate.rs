////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use tracing::{error, instrument};

#[derive(Debug, Deserialize)]
pub struct KeyRotateRsp {
    #[serde(alias = "QuerySpan")]
    pub query_span: u32,
    #[serde(alias = "PubKeyMeta")]
    pub pub_key_meta: KeyRotatePubKeyMetaRsp,
}

#[derive(Debug, Deserialize)]
pub struct KeyRotatePubKeyMetaRsp {
    #[serde(alias = "KeyVer")]
    pub key_ver: u8,
    #[serde(alias = "PubKey")]
    pub pub_key: String,
    #[serde(alias = "PubKeySign")]
    pub pub_key_sign: String,
}

#[inline]
pub async fn fetch_init_key() -> Result<KeyRotateRsp> { fetch_key(10000).await }

#[instrument]
pub async fn fetch_key(u: u32) -> Result<KeyRotateRsp> {
    let res = Client::new()
        .post("https://keyrotate.qq.com/rotate_key?cipher_suite_ver=305&uin=".to_string() + &u.to_string())
        .send()
        .await;
    if res.is_err() {
        error!(dsc = "请求错误",  err = %res.as_ref().unwrap_err());
    }

    let res = res?.json::<KeyRotateRsp>().await;
    if res.is_err() {
        error!(dsc = "解析错误",  err = %res.as_ref().unwrap_err());
    }

    Ok(res?)
}
