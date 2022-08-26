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
use tokio::try_join;
use tracing::{error, instrument};
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

use crate::network::protocol::server::get_http_server_list;

#[derive(Debug)]
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

    #[instrument]
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

    #[instrument]
    async fn fetch_server_by_protocol(&self) -> Result<Vec<SocketAddr>> {
        let s = get_http_server_list().await?;

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

#[cfg(test)]
mod tests {
    use super::ServerManager;

    #[tokio::test]
    async fn fetch_server_by_dns() {
        let mut a = ServerManager { socket: Vec::new(), quic: Vec::new() };
        let a = a.fetch_server_by_dns().await.unwrap();
        println!("{:#?}", a);
    }
}
