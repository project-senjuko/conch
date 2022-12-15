////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::Bytes;

use self::structs::Config;
use self::types::*;

mod structs;
mod config;
mod types;

/// 全局运行时变量
static mut RUNTIMEVAR: Option<&mut Runtime> = None;

/// 全局运行时
pub struct Runtime {
    config: Config,
    d2: Bytes,
    d2key: D2Key,
    tgt: Bytes,
    msg_cookie: Bytes,

    // 计数器等
}

impl Runtime {
    /// 初始化全局运行时变量
    pub fn init() {
        unsafe {
            RUNTIMEVAR = Some(Box::leak(Box::new(
                Runtime {
                    config: Config::read_config(),
                    d2: Default::default(),
                    d2key: Default::default(),
                    tgt: Default::default(),
                    msg_cookie: Default::default(),
                }
            )));
        }
    }

    /// 获取运行时变量
    ///
    /// # Safety
    ///
    /// 必须确保 [`Runtime::init`] 初始化全局运行时变量函数已被调用。
    fn get_var() -> &'static Runtime { unsafe { RUNTIMEVAR.as_ref().unwrap() } }

    /// 获取可变的运行时变量
    ///
    /// # Safety
    ///
    /// 必须确保 [`Runtime::init`] 初始化全局运行时变量函数已被调用。
    fn get_var_mut() -> &'static mut Runtime { unsafe { RUNTIMEVAR.as_mut().unwrap() } }
}

impl Runtime {
    /// 获取配置文件
    pub fn get_config() -> &'static Config { &Runtime::get_var().config }

    /// 获取 d2
    pub fn get_d2() -> &Bytes { &Runtime::get_var().d2 }

    /// 获取 d2key
    pub fn get_d2key() -> D2Key { Runtime::get_var().d2key }

    /// 获取 tgt
    pub fn get_tgt() -> &Bytes { &Runtime::get_var().tgt }

    /// 获取 msg_cookie
    pub fn get_msg_cookie() -> &Bytes { &Runtime::get_var().msg_cookie }
}

impl Runtime {
    /// 设置 d2
    pub fn put_d2(b: Bytes) { Runtime::get_var_mut().d2 = b }

    /// 设置 d2key
    pub fn put_d2key(d: D2Key) { Runtime::get_var_mut().d2key = d }

    /// 设置 tgt
    pub fn put_tgt(b: Bytes) { Runtime::get_var_mut().tgt = b }

    /// 设置 msg_cookie
    pub fn put_msg_cookie(b: Bytes) { Runtime::get_var_mut().msg_cookie = b }
}
