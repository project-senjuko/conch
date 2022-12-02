////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use serde::Deserialize;

pub use self::network::*;
pub use self::qq::*;

mod network;
mod qq;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub qq: QQTable,
    pub network: NetworkTable,
    pub misc: MiscTable,
}

#[derive(Debug, Deserialize)]
pub struct MiscTable {
    #[serde(rename = "startup-delay", default)]
    pub startup_delay: bool,
}
