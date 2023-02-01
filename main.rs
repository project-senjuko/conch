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
    axum_server::{Handle, tls_rustls::RustlsConfig},
    conch::runtime::Runtime,
    std::{io::Error, net::SocketAddr, time::Duration},
    tokio::time::sleep,
    tower_http::services::ServeFile,
    tracing::{info, instrument},
};

/// WELCOME TO CONCH
#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    // 准备启动 Conch 前初始化运行时
    Runtime::init().await;

    info!(
        dsc = "いらっしゃいません～",
        GITHUB = "https://github.com/qianjunakasumi/senjuko-conch",
        LICENSE = %Runtime::config().eula,
        PKGVersion = env!("CARGO_PKG_VERSION"),
        Branch = env!("GIT_BRANCH"),
        CommitHash = env!("GIT_HASH"),
        RustVersion = env!("RUST_VERSION"),
        BuildTime = env!("BUILD_TIME"),
    );

    if Runtime::config().misc.startup_delay {
        info!(dsc = "在正式启动前您有⑨秒预览配置文件～", cfg = ?Runtime::config());
        sleep(Duration::from_secs(9)).await;
    }

    Runtime::client_mut().boot().await;
    tokio::spawn(dashboard());

    info!(dsc = "うららか日和でしょでしょ～");
    Runtime::wait_stop().await;
    Runtime::client_mut().stop().await;
    info!(dsc = "プログラムは停止しますた、次回をお楽しみなのじゃ");
    Ok(())
}

/// 启动 Dashboard 服务
pub async fn dashboard() {
    let config = RustlsConfig::from_pem_file(
        Runtime::config().dashboard.cert.clone(),
        Runtime::config().dashboard.key.clone(),
    )
        .await
        .expect("证书文件错误");

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

    let addr = SocketAddr::from(
        ([0, 0, 0, 0], Runtime::config().dashboard.port)
    );
    info!(dsc = "Dashboard 服务启用", addr = %addr);

    let h = Handle::new();
    let ha = h.clone();

    tokio::spawn(async move {
        Runtime::wait_stop().await;
        ha.graceful_shutdown(Some(Duration::from_secs(9)));
    });

    axum_server::bind_rustls(addr, config)
        .handle(h)
        .serve(app.into_make_service())
        .await
        .expect("启动 Dashboard 服务失败");
}

async fn handle_error(err: Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
