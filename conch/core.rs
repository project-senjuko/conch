////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;
use tracing::{info, instrument};

use cookie::client::Client;
use cookie::runtime::Runtime;

/// 核心服务初始化
#[instrument]
pub async fn init_core() -> Result<()> {
    if Runtime::config().misc.startup_delay {
        info!(dsc = "默认情况下的正式启动前您有⑨秒预览配置文件，欲关闭此功能请在配置文件中设置 `startup-delay = false` 详见文档", cfg = ?Runtime::config());
        sleep(Duration::from_secs(9)).await;
    }

    let mut c = Client::new();
    c.run().await?;

    info!(dsc = "核心服务初始化成功");
    Ok(())
}
