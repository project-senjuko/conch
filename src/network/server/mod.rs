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

//! # 服务器管理器
//!
//! 发现、提供、更新服务器地址、协议。

use {
    self::{info::ServerInfo, protocol::server::fetch_server_list},
    anyhow::{bail, Result},
    crate::network::protocol,
    std::{
        net::{IpAddr, Ipv4Addr, SocketAddr},
        str::FromStr,
        thread::sleep,
        time::Duration,
    },
    tokio::{join, try_join},
    tracing::{debug, error, instrument, trace, warn},
    trust_dns_resolver::{
        config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts},
        TokioAsyncResolver,
    },
};

mod info;
mod r#static;

/// 服务器管理器
#[derive(Debug)]
pub struct ServerManager {
    /// 服务器列表
    server_list: Vec<ServerInfo>,
    /// 当前服务器索引
    current_index: usize,
}

impl ServerManager {
    #[inline]
    pub fn get_server_addr(&self) -> SocketAddr { self.server_list[self.current_index].socket_addr }

    pub fn next_server(&mut self) {
        if self.current_index == self.server_list.len() - 1 {
            warn!(dsc = "服务器列表索引值达到上限，触发冷却");
            // 冷却 9 秒，避免因重试服务器导致高 CPU 占用
            // 后续可以考虑异步刷新服务器列表
            sleep(Duration::from_secs(9));
            self.current_index = 0;
        }
        self.current_index += 1;
    }
}

impl ServerManager {
    #[inline]
    pub fn on_network_error(&mut self) {
        self.server_list[self.current_index].increasing_network_err();
    }

    #[inline]
    pub fn on_unreachable(&mut self) {
        self.server_list[self.current_index].set_unreachable();
    }

    #[inline]
    pub fn delay_sampling_cb(&mut self, d: u16) {
        self.server_list[self.current_index].set_delay_quality(d);
    }
}

impl ServerManager {
    /// 更新服务器列表
    #[instrument(skip(self))]
    pub async fn update_server_list(&mut self) -> Result<()> {
        let (pr, dr) = join!(
            self.fetch_server_by_protocol(),
            self.fetch_server_by_dns(),
        );
        if pr.is_err() && dr.is_err() {
            error!(
                dsc = "All 更新失败",
                protobufErr = %pr.as_ref().unwrap_err(), dnsErr = %dr.as_ref().unwrap_err(),
            );
            bail!("远程服务器列表获取失败");
        }

        if pr.is_ok() && dr.is_ok() {
            self.server_list.reserve(pr.as_ref().unwrap().len() + dr.as_ref().unwrap().len())
        }
        self.extend_server_list(pr, "Protobuf".to_string());
        self.extend_server_list(dr, "DNS".to_string());

        debug!(dsc = "成功");
        Ok(())
    }

    // 扩增服务器列表
    fn extend_server_list(&mut self, s: Result<Vec<ServerInfo>>, dsc: String) {
        match s {
            Ok(s) => { self.server_list.extend(s); }
            Err(e) => { warn!(dsc = dsc + " 更新失败", err = %e); }
        }
    }

    /// 通过 DNS 获取服务器列表
    #[instrument(skip(self))]
    async fn fetch_server_by_dns(&self) -> Result<Vec<ServerInfo>> {
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
            r.push(ServerInfo::with_tcp(SocketAddr::new(
                IpAddr::from(*v6re), 8080,
            )))
        }
        for v4re in v4res.iter() {
            r.push(ServerInfo::with_tcp(SocketAddr::new(
                IpAddr::from(*v4re), 8080,
            )))
        }

        Ok(r)
    }

    /// 通过 协议 获取服务器列表
    #[instrument(skip(self))]
    async fn fetch_server_by_protocol(&self) -> Result<Vec<ServerInfo>> {
        let s = fetch_server_list().await?;

        let mut r = Vec::new();
        for s in s.socket_wifi_ipv4.iter() {
            let i = IpAddr::from_str(&s.ip);
            if i.is_err() {
                if s.ip != "msfwifi.3g.qq.com" {
                    trace!(dsc = "解析 HttpServerListRes.socket_wifi_ipv4.ip 为 IpAddr 失败", err = %i.as_ref().unwrap_err(), ip = &*s.ip);
                }
                continue;
            }

            r.push(ServerInfo::with_tcp(SocketAddr::new(i?, s.port as u16)));
        }

        Ok(r)
    }
}
