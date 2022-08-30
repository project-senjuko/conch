////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::fs::read;

use anyhow::Result;
use tracing::{error, instrument};

use r#struct::Config;

mod r#struct;

/// 加载配置文件
#[instrument]
fn load_config(p: String) -> Result<Config> {
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
