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

//! 配置文件相关，
//! 定义配置文件的结构体和读取配置文件的方法。

use {
    self::{fields::*, tables::*},
    anyhow::Result,
    serde::Deserialize,
    super::env_or_default,
    tokio::fs,
    tracing::{debug, error, instrument, trace},
};

mod fields;
mod tables;

/// 配置文件结构
#[derive(Debug, Deserialize)]
pub struct Config {
    pub eula: License,
    pub dashboard: DashboardTable,
    pub data: DataTable,
    pub device: DeviceTable,
    pub network: NetworkTable,
    pub misc: MiscTable,
}

/// 配置文件读取实现
impl Config {
    /// 读取配置文件，
    /// 读取文件行为取决于环境变量 `SJKCONCH_CONFIG` 是否设置。
    ///
    /// ## 未设置环境变量
    ///
    /// 使用默认配置文件路径 `./Config.toml`，
    /// 即二进制执行文件同目录下的 `Config.toml` 文件。
    ///
    /// ## 已设置环境变量
    ///
    /// 读取该环境变量指示的文件，
    /// 若读取失败将抛出错误并停止程序。
    #[instrument]
    pub async fn read() -> Self {
        let p = env_or_default("SJKCONCH_CONFIG", "Config.toml");
        trace!(dsc = "读取配置文件", path = %p);

        let c = Config::read_vanilla(p).await;
        if c.is_ok() { debug!(dsc = "配置文件载入成功"); }

        c.expect("配置文件载入失败")
    }

    /// 读取配置文件
    #[instrument]
    async fn read_vanilla(p: String) -> Result<Config> {
        let b = fs::read_to_string(&p).await;
        if b.is_err() {
            error!(dsc = "读取失败", path = p, err = %b.as_ref().unwrap_err());
        }

        let c = toml::from_str(&b?);
        if c.is_err() {
            error!(dsc = "解析失败", err = %c.as_ref().unwrap_err())
        }

        Ok(c?)
    }
}
