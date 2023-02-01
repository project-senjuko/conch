////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use self::password::{Password, PasswordTrait};

mod password;

pub struct Secret {
    pub account: u32,
    pub password: Password,
}

impl Default for Secret {
    fn default() -> Self {
        Self {
            account: 0, // TODO 从持久化文件中读取
            password: Password::read_from_file(),
        }
    }
}
