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

//! # 客户端

use {
    crate::network::connection::ConnectionManager,
    crate::network::server::ServerManager,
};

/// 客户端
#[derive(Default)]
pub struct Client {
    server_manager: ServerManager,
    connection_manager: ConnectionManager,
}

/// 客户端
impl Client {
    /// 启动
    pub async fn boot(&mut self) {
        self.server_manager.update_server_list().await.expect("更新服务器列表失败");
    }

    /// 连接
    pub async fn connect(&mut self) {
        self.connection_manager.connect(&mut self.server_manager).await.expect("连接失败");
    }

    /// 停止
    pub async fn stop(&mut self) {
        //self.connection_manager.disconnect(&mut self.server_manager).await.expect("停止连接失败");
    }
}
