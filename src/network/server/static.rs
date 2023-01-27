////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::info::ServerInfo;
use super::ServerManager;

impl Default for ServerManager {
    fn default() -> Self {
        Self {
            server_list: vec![
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(183, 47, 102, 209)), 8080)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(113, 96, 18, 167)), 8080)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(157, 148, 36, 57)), 14000)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(120, 232, 67, 190)), 443)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(113, 96, 18, 167)), 14000)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(157, 148, 54, 73)), 443)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(120, 232, 19, 199)), 80)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(43, 154, 240, 21)), 8080)),
                ServerInfo::with_tcp(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(43, 154, 240, 194)), 8080)),
                ServerInfo::with_quic(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(58, 251, 106, 174)), 443)),
            ],
            current_index: 0,
            quality_disabled: false,
            quality_threshold: 0f32,
        }
    }
}
