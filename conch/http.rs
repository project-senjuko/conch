////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::net::SocketAddr;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, get_service};
use axum_extra::routing::SpaRouter;
use tokio::io::Error;
use tower_http::services::ServeFile;
use tracing::info;

async fn handle_error(err: Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

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
