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
    password: Password,
}

impl Default for Secret {
    fn default() -> Self {
        Self { password: Password::read_from_file() }
    }
}

impl Secret {
    pub fn password(&self) -> Password { self.password }
}
