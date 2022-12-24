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
use std::str::FromStr;

use anyhow::{bail, Result};
use tokio::{join, try_join};
use tracing::{debug, error, instrument, trace, warn};
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

pub use crate::network::protocol;

use self::info::ServerInfo;
use self::protocol::server::fetch_server_list;

mod info;
mod r#static;

/// 服务器管理器
#[derive(Debug)]
pub struct ServerManager {
    server_list: Vec<ServerInfo>,
    current_index: usize,

    quality_disabled: bool,
    quality_threshold: f32,
}

impl ServerManager {
    #[inline]
    pub fn get_server_addr(&self) -> SocketAddr { self.server_list[self.current_index].socket_addr }

    pub fn next_server(&mut self) {
        if self.server_list.len() - 1 == self.current_index {
            warn!(dsc = "所有服务器资源已耗尽，服务器质量评分功能已被禁用");
            self.quality_disabled = true;
            self.current_index = 0;
        }

        if !self.quality_disabled {
            let s = &self.server_list[self.current_index + 1];
            // todo 动态计算服务器评分 [0,1]
            let r = 1f32;
            if r < self.quality_threshold {
                debug!(dsc = "已忽略评分低于阈值的服务器");
                self.current_index += 1;
                self.next_server();
                return;
            }
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
