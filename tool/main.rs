////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use clap::{Parser, ValueHint, Subcommand};
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    /// 输入文件路径
    #[clap(short = 'i', long = "input")]
    #[clap(value_name = "输入文件路径")]
    #[arg(value_hint = ValueHint::FilePath)]
    input: Option<PathBuf>,
    /// 输出文件路径
    #[clap(short = 'o', long = "output")]
    #[clap(value_name = "输出文件路径")]
    #[arg(value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
    /// 工作目录
    #[clap(short = 'w', long = "workdir")]
    #[clap(value_name = "工作目录")]
    #[arg(value_hint = ValueHint::DirPath)]
    workdir: Option<PathBuf>,
    /// 子命令
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 输出版本信息
    Version,
}

fn main() {
    let cli = Cli::parse();

    let work_dir = env::current_dir().expect("Failed to get current workdir");
    let work_dir = cli.workdir.unwrap_or(work_dir);
    println!("当前工作目录为 {}", &work_dir.to_string_lossy());

    match &cli.command {
        Some(Commands::Version) => {
            // FIXME: 等待弃用 shadow-rs
        }
        None => {}
    }
}
