////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! 全局运行时

use {
    self::{config::Config, secret::Secret, types::*},
    bytes::Bytes,
    crate::client::Client,
    std::env,
    tokio::sync::watch::{channel, Receiver, Sender},
};

mod config;
pub mod lifecycle;
mod secret;
mod types;

/// 全局运行时变量
static mut RUNTIME: Option<&mut Runtime> = None;

/// 全局运行时
pub struct Runtime {
    client: Client,
    config: Config,
    secret: Secret,

    d2: Bytes,
    d2key: D2Key,
    tgt: Bytes,
    msg_cookie: Bytes,

    stop_signal_tx: Sender<bool>,
    stop_signal_rx: Receiver<bool>,

    // 计数器等
}

impl Runtime {
    /// 初始化全局运行时变量
    pub async fn init() {
        let (tx, rx) = channel::<bool>(false);

        unsafe {
            RUNTIME = Some(Box::leak(Box::new(
                Self {
                    client: Client::default(),
                    config: Config::read().await,
                    secret: Secret::default(),
                    d2: Default::default(),
                    d2key: Default::default(),
                    tgt: Default::default(),
                    msg_cookie: Default::default(),
                    stop_signal_tx: tx,
                    stop_signal_rx: rx,
                }
            )));
        }
    }

    /// 运行时变量
    ///
    /// # Safety
    ///
    /// 必须确保 [`Runtime::init`] 已被调用。
    fn rt() -> &'static Self { unsafe { RUNTIME.as_ref().unwrap() } }

    /// 可变运行时变量
    ///
    /// # Safety
    ///
    /// 必须确保 [`Runtime::init`] 已被调用。
    fn rt_mut() -> &'static mut Self { unsafe { RUNTIME.as_mut().unwrap() } }
}

impl Runtime {
    /// 客户端
    pub fn client() -> &'static Client { &Self::rt().client }

    /// 可变客户端
    pub fn client_mut() -> &'static mut Client { &mut Self::rt_mut().client }

    /// 配置
    pub fn config() -> &'static Config { &Self::rt().config }

    /// 机密
    pub fn secret() -> &'static Secret { &Self::rt().secret }

    /// 停机信号接收器
    pub async fn rx() {
        let _ = Self::rt().stop_signal_rx.clone().changed().await;
    }

    /// 等待停机
    pub async fn wait_stop() {
        wait_signal().await;
        Self::stop();
    }

    /// 停机
    pub fn stop() { Self::rt().stop_signal_tx.send(true).unwrap(); }

    // 考虑废弃以下特性（组合至其他部分）

    /// 获取 d2
    pub fn get_d2() -> Bytes { Runtime::rt().d2.clone() }

    /// 获取 d2key
    pub fn get_d2key() -> D2Key { Runtime::rt().d2key }

    /// 获取 tgt
    pub fn get_tgt() -> Bytes { Runtime::rt().tgt.clone() }

    /// 获取 msg_cookie
    pub fn get_msg_cookie() -> Bytes { Runtime::rt().msg_cookie.clone() }
}

impl Runtime {
    /// 设置 d2
    pub fn put_d2(b: Bytes) { Runtime::rt_mut().d2 = b }

    /// 设置 d2key
    pub fn put_d2key(d: D2Key) { Runtime::rt_mut().d2key = d }

    /// 设置 tgt
    pub fn put_tgt(b: Bytes) { Runtime::rt_mut().tgt = b }

    /// 设置 msg_cookie
    pub fn put_msg_cookie(b: Bytes) { Runtime::rt_mut().msg_cookie = b }
}

/// 获取环境变量，
/// 如果不存在则返回默认值。
pub fn env_or_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

#[cfg(unix)]
async fn wait_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    signal(SignalKind::terminate()).expect("监听 SIGTERM 信号失败").recv().await;
}

#[cfg(windows)]
async fn wait_signal() {
    use tokio::signal::ctrl_c;
    ctrl_c().await.expect("监听 Ctrl+C 信号失败");
}
