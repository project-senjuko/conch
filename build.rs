////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use shadow_rs::SdResult;
use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::Write;

const MAINTAINER_NAME: &str = "SJKCONCH_MAINTAINER_NAME";
const MAINTAINER_EMAIL: &str = "SJKCONCH_MAINTAINER_EMAIL";

fn main() -> SdResult<()> {
    shadow_rs::new_hook(hook)
}

fn hook(file: &File) -> SdResult<()> {
    append_build_info(file)?;
    Ok(())
}

fn append_build_info(file: &File) -> SdResult<()> {
    let maintainer_name: Cow<'static, str> = env::var(MAINTAINER_NAME)
        .map(Into::into)
        .unwrap_or_else(|_| "<unknown>".into());
    let maintainer_email: Cow<'static, str> = env::var(MAINTAINER_EMAIL)
        .map(Into::into)
        .unwrap_or_else(|_| "<unknown>".into());

    write_const(file, MAINTAINER_NAME, &maintainer_name)?;
    write_const(file, MAINTAINER_EMAIL, &maintainer_email)?;

    Ok(())
}

#[inline]
fn write_const(mut file: &File, const_name: &str, content: &str) -> std::io::Result<()> {
    writeln!(
        file,
        "{}",
        (String::from(r#"pub const "#) + const_name + r#": &str = ""# + content + r#"";"#)
    )
}
