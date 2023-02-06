////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! 生命周期模块，
//! 提供 Conch 生命周期内各类数据的持久化管理、读写，
//! 对上层暴露持久化内容的数据结构和操作方法。

use {
    anyhow::Result,
    std::path::{Path, PathBuf},
    super::Runtime,
    tokio::fs::{create_dir_all, write},
    tracing::{debug, instrument},
};

/// 激活
#[instrument]
pub async fn on_active() -> Result<()> {
    if !is_init() {
        init_create().await?;
        debug!(dsc = "初始化完成");
    }

    Ok(())
}

/// 是否已初始化
fn is_init() -> bool {
    Path::new(&Runtime::config().data.path).join("Initialized").exists()
}

/// 初始化
async fn init_create() -> Result<()> {
    create_dir_all(&Runtime::config().data.path).await?;
    write(
        Path::new(&Runtime::config().data.path).join("Initialized"),
        "",
    ).await?;

    // TODO: 初始化及随机化数据

    Ok(())
}

pub fn secret() -> PathBuf {
    Path::new(&Runtime::config().data.path).join("secret")
}
