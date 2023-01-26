////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//! 生命周期模块，
//! 提供 Conch 生命周期内各类数据的持久化管理、读写，
//! 对上层暴露持久化内容的数据结构和操作方法。

use std::path::Path;

use super::Runtime;

/// 是否已初始化
pub fn is_init() -> bool {
    Path::new(&Runtime::config().data.path).join("Initialized").exists()
}
