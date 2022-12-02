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

use super::network::server::ServerManager;

pub struct Client {
    server_manager: ServerManager,
}

impl Client {
    pub fn new() -> Self {
        Self { server_manager: Default::default() }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.server_manager.update_server_list().await
    }
}
