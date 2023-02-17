////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::net::SocketAddr;

/// 服务器信息
#[derive(Debug)]
pub struct ServerInfo {
    pub protocol: ServerProtocol,
    pub socket_addr: SocketAddr,

    unreachable: bool,
    network_error_count: u8,
    delay_quality: u16,
}

/// 服务器协议
#[derive(Debug, Eq, PartialEq)]
pub enum ServerProtocol { Tcp, Quic }

impl ServerInfo {
    /// 新建一个 [`ServerInfo`] 服务器信息
    pub fn new(sp: ServerProtocol, sd: SocketAddr) -> Self {
        Self { protocol: sp, socket_addr: sd, unreachable: false, network_error_count: 0, delay_quality: 0 }
    }

    /// 新建一个使用 TCP 协议的 [`ServerInfo`] 服务器信息
    #[inline]
    pub fn with_tcp(s: SocketAddr) -> Self { Self::new(ServerProtocol::Tcp, s) }

    /// 新建一个使用 QUIC 协议的 [`ServerInfo`] 服务器信息
    #[inline]
    pub fn with_quic(s: SocketAddr) -> Self { Self::new(ServerProtocol::Quic, s) }
}

impl ServerInfo {
    #[inline]
    pub fn increasing_network_err(&mut self) { self.network_error_count += 1; }

    #[inline]
    pub fn set_unreachable(&mut self) { self.unreachable = true }

    #[inline]
    pub fn set_delay_quality(&mut self, d: u16) { self.delay_quality = d }
}
