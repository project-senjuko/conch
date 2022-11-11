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
use tracing::{debug, error, instrument, trace};

pub use r#struct::*;

pub mod r#struct;

/// 全局配置
static mut CONFIG: Option<&Config> = None;

/// 加载配置文件到全局配置，
/// 读取文件行为取决于环境变量 `SJKCONCH_CONFIG` 是否设置。
/// 必须确保此函数在全局配置取值器函数调用前调用。
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
#[instrument]
pub fn load_config() -> Result<()> {
    let r = match var("SJKCONCH_CONFIG") {
        Ok(s) => {
            trace!(brc = "环境变量");
            _load_config(s)
        }
        Err(e) => {
            match e {
                VarError::NotPresent => {
                    trace!(brc = "默认位置");
                    _load_config("Config.toml".to_string())
                }
                VarError::NotUnicode(_) => {
                    const ERR: &str = "读取环境变量失败";
                    error!(dsc = ERR, err = %e);
                    Err(Error::msg(ERR))
                }
            }
        }
    };
    if r.is_ok() { debug!(dsc = "配置内容全局化完成"); }
    unsafe { CONFIG = Some(Box::leak(Box::new(r?))); }

    Ok(())
}

/// 加载配置文件的底层函数
#[instrument(skip(p))]
fn _load_config(p: String) -> Result<Config> {
    let b = read(&p);
    if b.is_err() {
        error!(dsc = "读取失败", path = p, err = %b.as_ref().unwrap_err());
    }

    let c = toml::from_slice(&*b?);
    if c.is_err() {
        error!(dsc = "解析失败", err = %c.as_ref().unwrap_err())
    }

    Ok(c?)
}

/// 获取配置
///
/// # Safety
///
/// 必须确保 [`load_config`] 函数已被调用。
pub fn get_config() -> &'static Config { unsafe { CONFIG.unwrap() } }
