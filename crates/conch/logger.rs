////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use tracing::{info, instrument};
use tracing_subscriber::{EnvFilter, filter, fmt, layer, prelude::*, Registry, reload};
use shadow_rs::shadow;

shadow!(build);

/// 日志记录器初始化
#[instrument]
pub fn init_logger() -> reload::Handle<EnvFilter, layer::Layered<fmt::Layer<Registry>, Registry>> {
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
        BuildRustChannel = build::BUILD_RUST_CHANNEL
    );

    h
}
