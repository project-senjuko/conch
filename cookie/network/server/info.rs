////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::net::SocketAddr;

/// 服务器信息
#[derive(Debug)]
pub struct ServerInfo {
    pub protocol: ServerProtocol,
    pub socket_addr: SocketAddr,
}

/// 服务器协议
#[derive(Debug, Eq, PartialEq)]
pub enum ServerProtocol { Tcp, Quic }

impl ServerInfo {
    /// 新建一个 [`ServerInfo`] 服务器信息
    pub fn new(sp: ServerProtocol, sd: SocketAddr) -> Self {
        Self { protocol: sp, socket_addr: sd }
    }

    /// 新建一个使用 TCP 协议的 [`ServerInfo`] 服务器信息
    #[inline(always)]
    pub fn with_tcp(s: SocketAddr) -> Self { Self::new(ServerProtocol::Tcp, s) }

    /// 新建一个使用 QUIC 协议的 [`ServerInfo`] 服务器信息
    #[inline(always)]
    pub fn with_quic(s: SocketAddr) -> Self { Self::new(ServerProtocol::Quic, s) }
}
