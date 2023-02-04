////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

type B16 = [u8; 16];

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub account: u32,
    pub password: B16,
    pub tgtgt: B16,
}

impl Secret {
    pub async fn read() -> Self {

        // TODO 删除以下实现，利用 Lifecycle 模块读取数据并解序列化
        Self {
            account: 0,
            password: [0; 16],
            tgtgt: [0; 16],
        }
    }
}
