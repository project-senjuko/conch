////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use anyhow::Result;
use shadow_rs::shadow;
use tokio::spawn;
use tracing::{info, instrument};

use cookie::runtime::Runtime;

use self::core::init_core;
use self::http::dashboard;
use self::logger::init_logger;

mod logger;
mod core;
mod http;

shadow!(build);

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
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

    init_core().await.expect("核心服务初始化失败");
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
