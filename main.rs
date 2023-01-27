////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use {
    anyhow::Result,
    axum::{http::StatusCode, response::IntoResponse, Router, routing::{get, get_service}},
    axum_extra::routing::SpaRouter,
    cookie::runtime::Runtime,
    shadow_rs::shadow,
    std::{io::Error, net::SocketAddr, time::Duration},
    tokio::{spawn, time::sleep},
    tower_http::services::ServeFile,
    tracing::{info, instrument},
    tracing_subscriber::{EnvFilter, filter, fmt, layer, prelude::*, Registry, reload},
};

shadow!(build);

/// WELCOME TO CONCH
#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    // 准备启动 Conch 前初始化运行时
    Runtime::init().await;

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

/// 启动 Dashboard 服务
pub async fn dashboard() {
    let app = Router::new()
        .route(
            "/conch-cgi/hello",
            get(|| async { "Conch 海螺 Dashboard 服务已正确运行" }),
        )
        .route(
            "/favicon.svg",
            get_service(ServeFile::new("dashboard/favicon.svg"))
                .handle_error(handle_error),
        )
        .merge(
            SpaRouter::new("/assets", "dashboard/assets")
                .index_file("../index.html")
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 1919));
    info!(dsc = "Dashboard 服务启用", addr = %addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(err: Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// 日志记录器初始化
#[instrument]
pub fn init_logger() -> (String, reload::Handle<EnvFilter, layer::Layered<fmt::Layer<Registry>, Registry>>) {
    let e = EnvFilter::builder()
        .with_default_directive(filter::LevelFilter::INFO.into())
        .with_env_var("SJKCONCH_LOG")
        .from_env_lossy();
    let lev = e.to_string();
    let (lay, h) = reload::Layer::new(e);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(lay)
        .init();

    (lev, h)
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
