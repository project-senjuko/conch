////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use anyhow::Result;
use bytes::BytesMut;
use tracing::{error, instrument};

use jce::field::{JceFieldErr, JLong};

use crate::config::app_setting::APP_ID;
use crate::network::protocol::jce::r#struct::{HttpServerListReq, HttpServerListRes};
use crate::network::protocol::jce::r#struct::UniPacket;

const KEY: [u32; 4] = [4030996319, 4096632207, 3707212954, 3127038993];

const ERR: &str = "请求服务器列表失败";

/// 通过 HTTP 获取服务器列表
#[instrument]
pub async fn fetch_server_list() -> Result<HttpServerListRes> {
    let mut p = UniPacket::new(0, "HttpServerListReq", "HttpServerListReq");
    p.put("HttpServerListReq", HttpServerListReq {
        timeout: 60,
        c: 1,
        is_wifi_conn: 100,
        app_id: APP_ID as JLong,
        l: Some(1),
        ..Default::default()
    });

    let res = reqwest::Client::new()
        .post("https://configsvr.msf.3g.qq.com/configsvr/serverlist.jsp?mType=getssolist")
        .body(p.encode_with_tea(KEY).freeze())
        .send()
        .await;
    if res.is_err() {
        error!(msg = ERR, "网络请求原因：{}", res.as_ref().err().unwrap());
    }
    let res = res?.bytes().await;
    if res.is_err() {
        error!(msg = ERR, "网络读取原因：{}", res.as_ref().err().unwrap());
    }

    let p = UniPacket::from(BytesMut::from(res?.as_ref()), KEY);
    if p.is_err() {
        error!(msg = ERR, "Jce包 解析原因：{}", p.as_ref().err().unwrap());
    }
    let d: Result<HttpServerListRes, JceFieldErr> = p?.get("HttpServerListRes");
    if d.is_err() {
        error!(msg = ERR, "Jce结构体 解析原因：{}", d.as_ref().err().unwrap());
    }

    Ok(d?)
}
