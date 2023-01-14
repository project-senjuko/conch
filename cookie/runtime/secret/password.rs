////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

//use std::fs;

pub type Password = [u8; 16];

pub trait PasswordTrait {
    fn read_from_file() -> Password;
}

impl PasswordTrait for Password {
    fn read_from_file() -> Password {
        //fs::read("")
        // 不存在时退出
        //todo!()
        Password::default()
    }
}
