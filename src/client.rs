////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use crate::{
    network::server::ServerManager,
    runtime::lifecycle,
};

#[derive(Default)]
pub struct Client {
    server_manager: ServerManager,
}

impl Client {
    pub async fn boot(&mut self) {
        self.server_manager.update_server_list().await.expect("更新服务器列表失败");

        if !lifecycle::is_init() {
            // 未完成初始化
            // TODO: 初始化
        }
    }
}
