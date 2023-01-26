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
use shadow_rs::shadow;
use tokio::{spawn, time::sleep};
use tracing::{info, instrument};

use cookie::runtime::Runtime;

use self::{http::dashboard, logger::init_logger};

mod http;
mod logger;

shadow!(build);

/// WELCOME TO CONCH
#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    // 准备启动 Conch 前初始化运行时
    Runtime::init();

    let (lev, _h) = init_logger(); // _h 用于 dashboard 和 gRPC 动态切换日志等级

    info!(
        dsc = "いらっしゃいません～",
        PROJECT = "Project Senjuko - Conch 海螺",
        GITHUB = "https://github.com/qianjunakasumi/senjuko-conch",
        LICENSE = ?Runtime::config().eula,
        COPYRIGHT = "Copyright (C) 2022-2023  qianjunakasumi <i@qianjunakasumi.ren>",
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

    if Runtime::config().misc.startup_delay {
        info!(dsc = "在正式启动前您有⑨秒预览配置文件～", cfg = ?Runtime::config());
        sleep(Duration::from_secs(9)).await;
    }

    Runtime::client_mut().boot().await;
    spawn(dashboard());

    info!(dsc = "うららか日和でしょでしょ～");
    wait_signal().await;
    // 通知停机
    info!(dsc = "プログラムは停止しますた、次回をお楽しみなのじゃ");
    Ok(())
}

#[cfg(unix)]
async fn wait_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    signal(SignalKind::terminate()).expect("监听 SIGTERM 信号失败").recv().await;
}

#[cfg(windows)]
async fn wait_signal() {
    use tokio::signal::ctrl_c;
    ctrl_c().await.expect("监听 Ctrl+C 信号失败");
}
