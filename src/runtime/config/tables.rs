////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataTable {
    #[serde(default)] pub path: String,
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
    #[serde(rename = "enable-ipv6", default)]
    pub enable_ipv6: bool,
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

#[derive(Debug, Deserialize)]
pub struct DeviceTable {
    #[serde(default)] pub model: String,
    #[serde(default)] pub manufacturer: String,
    #[serde(default)] pub board: String,
    #[serde(default)] pub device: String,
    #[serde(default)] pub product: String,
    #[serde(default)] pub ssid: String,
    #[serde(default)] pub ipv4: String,
    #[serde(default)] pub ipv6: String,
    #[serde(default)] pub mac: String,
    #[serde(default)] pub fingerprint: String,
    #[serde(default)] pub imei: String,
    #[serde(default)] pub imsi: String,
    #[serde(rename = "system-http-ua", default)] pub system_http_ua: String,
}

#[derive(Debug, Deserialize)]
pub struct MiscTable {
    #[serde(rename = "startup-delay", default)]
    pub startup_delay: bool,
}
