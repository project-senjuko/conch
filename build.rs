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
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> SdResult<()> {
    shadow_rs::new_hook(hook)
}

fn hook(file: &File) -> SdResult<()> {
    append_maintainer_info(file)?;
    Ok(())
}

fn append_maintainer_info(mut file: &File) -> SdResult<()> {
    let maintainer_name = env::var("SJKCONCH_MAINTAINER_NAME");
    let maintainer_email = env::var("SJKCONCH_MAINTAINER_EMAIL");
    writeln!(
        file,
        "{}",
        String::from(r#"pub const SJKCONCH_MAINTAINER_NAME: &str = ""#)
            + (if maintainer_name.is_err() {
                "<unknown>"
            } else {
                maintainer_name.as_ref().unwrap()
            })
            + r#"";"#
    )?;
    writeln!(
        file,
        "{}",
        String::from(r#"pub const SJKCONCH_MAINTAINER_EMAIL: &str = ""#)
            + (if maintainer_email.is_err() {
                "<unknown>"
            } else {
                maintainer_email.as_ref().unwrap()
            })
            + r#"";"#
    )?;
    Ok(())
}
