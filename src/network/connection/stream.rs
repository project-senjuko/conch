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

use {
    anyhow::{bail, Result},
    bytes::BytesMut,
    std::net::SocketAddr,
    tokio::net::TcpStream,
    tracing::{trace, warn},
};

/// 连接
pub(super) struct ConnectionStream {
    //pub uuid
    //pub left
    //pub quality
    stream: TcpStream,
    // RX
    // TX
}

impl ConnectionStream {
    /// 新建一个连接
    pub async fn new(server_addr: SocketAddr) -> Result<Self> {
        // 考虑连接时间
        let stream = TcpStream::connect(server_addr).await?;
        trace!(dsc = "connected to server", addr = ?server_addr);
        Ok(Self { stream })
    }

    /// 监听
    pub async fn listen(&mut self) -> Result<()> {
        trace!(dsc = "listening", addr = %self.stream.peer_addr()?);
        loop {
            self.stream.readable().await?;

            let msg_len = match self.read_msg_len() {
                Ok(s) => s,
                Err(e) => {
                    warn!(dsc = "报文长度标头错误", err = %e);
                    continue;
                }
            };
            let _msg = match self.read_msg(msg_len) {
                Ok(s) => s,
                Err(e) => {
                    warn!(dsc = "报文读取错误", err = %e);
                    continue;
                }
            };
            // 报文通过管道发送给处理器
        }
    }

    /// 获取远程地址
    pub fn peer_addr(&self) -> Result<SocketAddr> { Ok(self.stream.peer_addr()?) }

    /// 读取报文长度
    #[inline]
    fn read_msg_len(&self) -> Result<usize> {
        let mut l = [0u8; 4];
        match self.stream.try_read(&mut l) {
            Ok(n) => {
                if n != 4 { bail!("bad message length"); }
                Ok(u32::from_be_bytes(l) as usize - 4) // 报文长度含自身
            }
            Err(e) => Err(e.into()),
        }
    }

    /// 读取报文
    #[inline]
    fn read_msg(&self, len: usize) -> Result<BytesMut> {
        let mut msg = BytesMut::with_capacity(len);
        match self.stream.try_read(&mut msg) {
            Ok(_) => Ok(msg),
            Err(e) => Err(e.into()),
        }
    }
}
