////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use std::env;
use std::fs::File;
use std::io::Write;
use shadow_rs::SdResult;

fn main() -> SdResult<()> {
    shadow_rs::new_hook(hook)
}

fn hook(file: &File) -> SdResult<()> {
    append_maintainer_info(file)?;
    Ok(())
}

fn append_maintainer_info(mut file: &File) -> SdResult<()> {
    let maintainer_name = env::var("SJKCONCH_MAINTAINER_NAME")?;
    let maintainer_email = env::var("SJKCONCH_MAINTAINER_EMAIL")?;
    writeln!(file, "{}", String::from(r#"pub const SJKCONCH_MAINTAINER_NAME: &str = ""#) + &maintainer_name + r#"";"#)?;
    writeln!(file, "{}", String::from(r#"pub const SJKCONCH_MAINTAINER_EMAIL: &str = ""#) + &maintainer_email + r#"";"#)?;
    Ok(())
}
