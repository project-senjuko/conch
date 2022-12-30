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
