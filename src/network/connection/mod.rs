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

//! # 连接管理器
//!
//! 管理连接生命周期，处理连接的创建、监听、断开、重连等。

use {
    anyhow::Result,
    crate::network::server::ServerManager,
    self::stream::ConnectionStream,
    tracing::info,
};

mod stream;

#[derive(Default)]
pub struct ConnectionManager {
    connection: Option<ConnectionStream>,
}

impl ConnectionManager {
    pub async fn connect(&mut self, sm: &mut ServerManager) -> Result<()> {
        let sa = sm.get_server_addr();
        match ConnectionStream::new(sa).await {
            Ok(c) => {
                self.connection = Some(c);
                info!(dsc = "连接到服务器", addr = %sa);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // 读取连接信息等
}
