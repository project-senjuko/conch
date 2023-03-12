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

use std::env;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use crate::commands::login_packet::parse_login_packet;

use crate::commands::version::print_version_info;

mod commands;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
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
pub enum Commands {
    /// 输出版本信息
    Version,
    /// 解析登录包
    LoginPacket {
        #[arg(value_name = "输入文件路径")]
        #[arg(value_hint = ValueHint::DirPath)]
        input: PathBuf,
        #[arg(value_name = "ShareKey")]
        shareKey: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let work_dir = env::current_dir().expect("Failed to get current workdir");
    let work_dir = cli.workdir.unwrap_or(work_dir);
    println!("当前工作目录为 {}", &work_dir.to_string_lossy());

    match &cli.command {
        Some(Commands::Version) => {
            print_version_info();
        }
        Some(Commands::LoginPacket { input, shareKey }) => {
            parse_login_packet(input, shareKey);
        }
        None => {}
    }
}
