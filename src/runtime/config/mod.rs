////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! 配置文件相关，
//! 定义配置文件的结构体和读取配置文件的方法。

use {
    self::{fields::*, tables::*},
    anyhow::{anyhow, Result},
    serde::Deserialize,
    std::env::{var, VarError},
    tokio::fs,
    tracing::{debug, error, instrument, trace},
};

mod fields;
mod tables;

/// 配置文件结构
#[derive(Debug, Deserialize)]
pub struct Config {
    pub eula: License,
    pub data: DataTable,
    pub qq: QQTable,
    pub network: NetworkTable,
    pub device: DeviceTable,
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
    /// 若读取失败将抛出错误并导致程序停止。
    #[instrument]
    pub async fn read_config() -> Self {
        let r = match var("SJKCONCH_CONFIG") {
            Ok(s) => {
                trace!(brc = "环境变量");
                Config::read_config_vanilla(s).await
            }
            Err(e) => {
                match e {
                    VarError::NotPresent => {
                        trace!(brc = "默认位置");
                        Config::read_config_vanilla("Config.toml".to_string()).await
                    }
                    VarError::NotUnicode(_) => {
                        const ERR: &str = "读取环境变量失败";
                        error!(dsc = ERR, err = %e);
                        Err(anyhow!(ERR))
                    }
                }
            }
        };
        if r.is_ok() { debug!(dsc = "配置内容全局化完成"); }

        r.unwrap()
    }

    /// 读取配置文件
    #[instrument]
    async fn read_config_vanilla(p: String) -> Result<Config> {
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
