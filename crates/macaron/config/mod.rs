////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::env::{var, VarError};
use std::fs::read;

use anyhow::{Error, Result};
use tracing::{error, instrument};

use r#struct::Config;

mod r#struct;

/// 加载配置文件，
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
/// 若读取失败将抛出错误，可能会导致程序停止。
#[instrument(fields(act = "加载配置文件"))]
pub fn load_config() -> Result<Config> {
    let v = var("SJKCONCH_CONFIG");
    match v {
        Ok(s) => { _load_config(s) }
        Err(e) => {
            match e {
                VarError::NotPresent => { _load_config("Config.toml".to_string()) }
                VarError::NotUnicode(_) => {
                    const ERR: &str = "读取环境变量中配置文件路径失败";
                    error!(dsc = ERR, err = %e);
                    Err(Error::msg(ERR))
                }
            }
        }
    }
}

#[instrument]
fn _load_config(p: String) -> Result<Config> {
    let b = read(&p);
    if b.is_err() {
        error!(
            dsc = "读取配置文件失败", path = p,
            err = %b.as_ref().unwrap_err(),
        );
    }

    let c = toml::from_slice(&*b?);
    if c.is_err() {
        error!(dsc = "解析配置文件失败", err = %c.as_ref().unwrap_err())
    }

    Ok(c?)
}
