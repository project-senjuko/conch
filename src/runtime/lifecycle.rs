////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

//! 生命周期模块，
//! 提供 Conch 生命周期内各类数据的持久化管理、读写，
//! 对上层暴露持久化内容的数据结构和操作方法。

use {
    once_cell::sync::Lazy,
    std::path::{Path, PathBuf},
    super::env_or_default,
    tokio::fs::create_dir_all,
};

/// 数据目录，
/// 默认为当前目录下的 goconch 文件夹，可通过环境变量 SJKCONCH_DATA_PATH 修改。
static DATA_PATH: Lazy<String> = Lazy::new(
    || env_or_default("SJKCONCH_DATA_PATH", "goconch"),
);

/// 生命周期开始
pub async fn life_start() {
    create_dir_all(&*DATA_PATH).await.expect("创建数据目录失败");
}

/// 机密
pub fn secret() -> PathBuf {
    Path::new(&*DATA_PATH).join("secret")
}
