////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime, UtcOffset};

pub fn print_version_info() {
    let build_time = OffsetDateTime::from_unix_timestamp(env!("BUILD_TIME").parse().unwrap())
        .expect("解析构建时间失败")
        .to_offset(UtcOffset::current_local_offset().expect("获取系统时区失败"));

    println!(
        "ConchTool v{version} · {git_hash}\n\
    {build_time}
    ",
        version = env!("CARGO_PKG_VERSION"),
        git_hash = &env!("GIT_HASH")[..10],
        build_time = build_time.format(&Rfc3339).expect("解析构建时间失败")
    )
}
