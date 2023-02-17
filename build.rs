////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{process::Command, time::{SystemTime, UNIX_EPOCH}};

fn main() {
    println!(
        "cargo:rustc-env=GIT_HASH={}",
        String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "HEAD"])
                .output().unwrap().stdout
        ).unwrap(),
    );

    println!(
        "cargo:rustc-env=GIT_BRANCH={}",
        String::from_utf8(
            Command::new("git")
                .args(["rev-parse", "--abbrev-ref", "HEAD"])
                .output().unwrap().stdout,
        ).unwrap(),
    );

    println!(
        "cargo:rustc-env=RUST_VERSION={}",
        String::from_utf8(
            Command::new("rustc")
                .args(["--version"])
                .output().unwrap().stdout,
        ).unwrap(),
    );

    println!(
        "cargo:rustc-env=BUILD_TIME={}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    );
}
