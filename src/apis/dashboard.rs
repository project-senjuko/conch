////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use {
    async_graphql::{Error, Object, Result},
    tracing::trace,
};

pub struct MutDashboard;

#[Object]
impl MutDashboard {
    async fn login(&self, uin: u32, password: String) -> Result<bool> {
        trace!(dsc = "request login from dashboard", uin, password);
        Err(Error::from("soon.."))
    }
}
