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
use tokio_graceful_shutdown::SubsystemHandle;
use tracing::{info, instrument};

use cookie::client::Client;
use cookie::config::load_config;

/// 核心服务初始化
#[instrument(skip(sh))]
pub async fn init_core(sh: SubsystemHandle) -> Result<()> {
    load_config().expect("加载配置内容失败");

    let mut c = Client::new();
    c.run().await.expect("核心服务初始化失败");

    info!(dsc = "核心服务初始化成功");

    sh.on_shutdown_requested().await;
    // 请求关闭服务器
    Ok(())
}
