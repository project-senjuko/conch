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
use shadow_rs::shadow;
use tokio::signal::ctrl_c;
use tracing::{error, info, instrument, warn};

use self::core::init_core;
use self::logger::init_logger;

mod logger;
mod core;

shadow!(build);

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    let (lev, _h) = init_logger(); // _h 用于 dashboard 和 gRPC 动态切换日志等级

    info!(
        dsc = "いらっしゃいません！せんじゅうこコンチプロジェクトいます！ 今、進行中なので、少し我慢してくださいね？",
        LogLevel = lev,
        PKGVersion = build::PKG_VERSION,
        Branch = build::BRANCH,
        CommitHash = build::COMMIT_HASH,
        CommitDate = build::COMMIT_DATE_3339,
        CommitAuthor = build::COMMIT_AUTHOR,
        CommitEmail = build::COMMIT_EMAIL,
        BuildOS = build::BUILD_OS,
        BuildTarget = build::BUILD_TARGET,
        RustVersion = build::RUST_VERSION,
        BuildTime = build::BUILD_TIME_3339,
        BuildRustChannel = build::BUILD_RUST_CHANNEL,
        SJKConchMaintainerName = build::SJKCONCH_MAINTAINER_NAME,
        SJKConchMaintainerEmail = build::SJKCONCH_MAINTAINER_EMAIL,
    );

    match init_core().await {
        Ok(_) => { info!(dsc = "核心服务初始化成功") }
        Err(_) => { panic!("核心服务初始化失败！请检查错误日志并解决后再行启动") }
    }

    match ctrl_c().await {
        Ok(()) => { warn!(dsc = "收到退出请求，开始清理") }
        Err(e) => { error!(dsc = "监听退出请求失败", err = %e) }
    }

    // 通知各模块停止服务

    info!(dsc = "プログラムは停止しますた、次回をお楽しみにじゃ");
    Ok(())
}
