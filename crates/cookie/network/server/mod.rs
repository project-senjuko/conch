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
use std::str::FromStr;

use anyhow::Result;
use tokio::{join, try_join};
use tracing::{error, instrument, warn};
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

use crate::network::protocol::server::fetch_server_list;

mod r#static;

/// 服务器管理器
#[derive(Debug)]
pub struct ServerManager {
    socket: Vec<SocketAddr>,
    quic: Vec<SocketAddr>,
}

impl ServerManager {
    /// 更新服务器列表
    #[instrument(skip(self))]
    pub async fn update_server_list(&mut self) -> Result<()> {
        let r = join!(
            self.fetch_server_by_protocol(),
            self.fetch_server_by_dns(),
        );
        if r.0.is_err() && r.1.is_err() {
            error!(
                dsc = "无法通过 网络 更新服务器列表",
                protobufErr = %r.0.as_ref().unwrap_err(), dnsErr = %r.1.as_ref().unwrap_err(),
            );
            return Err(r.0.unwrap_err());
        }

        match r.0 {
            Ok(s) => { self.socket.extend_from_slice(&*s); }
            Err(e) => { warn!(dsc = "无法通过 Protobuf 更新服务器列表", err = %e); }
        }
        match r.1 {
            Ok(s) => { self.socket.extend_from_slice(&*s); }
            Err(e) => { warn!(dsc = "无法通过 DNS 更新服务器列表", err = %e); }
        }

        Ok(())
    }

    /// 通过 DNS 获取服务器列表
    #[instrument(skip(self))]
    async fn fetch_server_by_dns(&self) -> Result<Vec<SocketAddr>> {
        let mut rc = ResolverConfig::new();
        rc.add_name_server(NameServerConfig {
            socket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(119, 29, 29, 29)), 53), // DNSPod
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_nx_responses: true,
            bind_addr: None,
        });

        let r = TokioAsyncResolver::tokio(rc, ResolverOpts::default());
        if r.is_err() {
            error!(dsc = "初始化 DNS Resolver 失败", err = %r.as_ref().unwrap_err());
        }
        let r = r?;

        let res = try_join!(r.ipv6_lookup("msfwifiv6.3g.qq.com"), r.ipv4_lookup("msfwifi.3g.qq.com"));
        if res.is_err() {
            error!(dsc = "通过 DNS 获取服务器地址失败", err = %res.as_ref().unwrap_err());
        }

        let (v6res, v4res) = res?;
        let mut r = Vec::with_capacity(v6res.iter().count() + v4res.iter().count());

        for v6re in v6res.iter() {
            r.push(SocketAddr::new(IpAddr::from(*v6re), 8080))
        }
        for v4re in v4res.iter() {
            r.push(SocketAddr::new(IpAddr::from(*v4re), 8080))
        }

        Ok(r)
    }

    /// 通过 协议 获取服务器列表
    #[instrument(skip(self))]
    async fn fetch_server_by_protocol(&self) -> Result<Vec<SocketAddr>> {
        let s = fetch_server_list().await?;

        let mut r = Vec::new();
        for s in s.socket_wifi_ipv4.iter() {
            let i = IpAddr::from_str(&*s.ip);
            if i.is_err() {
                error!(dsc = "解析 HttpServerListRes.socket_wifi_ipv4.ip 为 IpAddr 失败", err = %i.as_ref().unwrap_err());
            }

            r.push(SocketAddr::new(i?, s.port as u16));
        }

        Ok(r)
    }
}
