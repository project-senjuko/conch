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

#[derive(Debug, Deserialize)]
pub struct Config {
    pub qq: QQTable,
    pub network: NetworkTable,
}

#[derive(Debug, Deserialize)]
pub struct QQTable {
    pub account: QQAccountTable,
}

#[derive(Debug, Deserialize)]
pub struct QQAccountTable {
    pub number: u64,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkTable {
    #[serde(default = "network_table_dns_default")]
    pub dns: Vec<NetworkDNSTable>,
}

fn network_table_dns_default() -> Vec<NetworkDNSTable> {
    vec![
        NetworkDNSTable { address: String::from("119.29.29.29") }
    ]
}

#[derive(Debug, Deserialize)]
pub struct NetworkDNSTable {
    pub address: String,
}
