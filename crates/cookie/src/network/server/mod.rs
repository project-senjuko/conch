////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct ServerManager {
    socket: Vec<SocketAddr>,
    quic: Vec<SocketAddr>,
    // "socket://msfwifiv6.3g.qq.com:8080"
    // "socket://msfwifi.3g.qq.com:8080"
}

impl ServerManager {
    fn new() -> ServerManager {
        ServerManager {
            socket: vec![
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(183, 47, 102, 209)), 8080),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(113, 96, 18, 167)), 8080),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(157, 148, 36, 57)), 14000),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(120, 232, 67, 190)), 443),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(113, 96, 18, 167)), 14000),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(157, 148, 54, 73)), 443),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(120, 232, 19, 199)), 80),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(43, 154, 240, 21)), 8080),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(43, 154, 240, 194)), 8080),
            ],
            quic: vec![
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(58, 251, 106, 174)), 443),
            ],
        }
    }
}