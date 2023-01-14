////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, ValueHint};
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 输入文件路径
    #[clap(short = 'i', long = "input")]
    #[clap(value_name = "INPUT")]
    #[arg(value_hint = ValueHint::FilePath)]
    input: PathBuf,
    /// 输出文件路径
    #[clap(short = 'o', long = "output")]
    #[clap(value_name = "OUTPUT")]
    #[arg(value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let work_dir = env::current_dir().expect("Failed to get current workdir");
    let work_dir = cli.work_dir.unwrap_or(work_dir);
    println!("当前工作目录为 {}", &work_dir.to_string_lossy())
}
