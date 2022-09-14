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
use tokio;
use tracing::{info, instrument};

use self::core::init_core;
use self::logger::init_logger;

mod logger;
mod core;

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    let _h = init_logger(); // _h 用于 dashboard 和 gRPC 动态切换日志等级
    match init_core().await {
        Ok(_) => { info!(dsc = "核心服务初始化成功") }
        Err(_) => { panic!("核心服务初始化失败！请检查错误日志并解决后再行启动") }
    }

    Ok(())
}
