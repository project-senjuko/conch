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

use super::config::Config;
use super::network::server::ServerManager;

pub struct Client {
    server_manager: ServerManager,
    config: Config,
}

impl Client {
    pub fn new(c: Config) -> Self {
        Self { server_manager: Default::default(), config: c }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.server_manager.update_server_list().await
    }
}
