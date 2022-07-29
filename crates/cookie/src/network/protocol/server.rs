////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::Bytes;

use jce::field::JLong;
use jce::packet::JcePacketV3;

use crate::config::app_setting::APP_ID;
use crate::network::protocol::r#struct::jce::{HttpServerListReq, HttpServerListRes};

const KEY: [u32; 4] = [4030996319, 4096632207, 3707212954, 3127038993];

/// 获取 HTTP 服务器列表
pub async fn get_http_server_list() -> HttpServerListRes {
    let mut p = JcePacketV3::new(0, "HttpServerListReq", "HttpServerListReq");
    p.put("HttpServerListReq", HttpServerListReq {
        timeout: 60,
        c: 1,
        is_wifi_conn: 100,
        app_id: APP_ID as JLong,
        l: Some(1),
        ..Default::default()
    });

    let mut p = JcePacketV3::from(
        &mut reqwest::Client::new()
            .post("https://configsvr.msf.3g.qq.com/configsvr/serverlist.jsp?mType=getssolist")
            .body(Bytes::from(
                p.encode_with_tea(KEY)
            ))
            .send()
            .await
            .expect("请求服务器列表失败") //TODO log打印
            .bytes()
            .await
            .expect("读取服务器列表失败"), //TODO
        KEY,
    );
    p.get("HttpServerListRes")
}

#[cfg(test)]
mod tests {
    use super::get_http_server_list;

    #[tokio::test]
    async fn to_bytes() {
        get_http_server_list().await;
    }
}
