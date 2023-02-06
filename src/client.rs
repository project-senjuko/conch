////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! 客户端，

use crate::network::server::ServerManager;

/// 客户端
#[derive(Default)]
pub struct Client {
    server_manager: ServerManager,
}

/// 客户端
impl Client {
    /// 启动
    pub async fn boot(&mut self) {
        self.server_manager.update_server_list().await.expect("更新服务器列表失败");
    }

    /// 停止
    pub async fn stop(&mut self) {
        // 停止
    }
}
