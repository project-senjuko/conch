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
use tokio_graceful_shutdown::Toplevel;
use tracing::{info, instrument};

use cookie::runtime::Runtime;

use self::core::init_core;
use self::logger::init_logger;

mod logger;
mod core;

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
        LICENSES = ?Runtime::get_config().eula,
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

    Toplevel::new()
        .start("core", init_core)
        .catch_signals()
        .handle_shutdown_requests(Duration::from_secs(3))
        .await
        .expect("启动服务失败");

    info!(dsc = "プログラムは停止しますた、次回をお楽しみなのじゃ");
    Ok(())
}
