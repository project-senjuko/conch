////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use anyhow::{bail, Result};
use bytes::{Buf, BytesMut};
use tracing::{error, instrument, trace};

use crate::cipher::qtea::QTeaCipher;
use crate::network::protocol::transport::Flag;
use crate::runtime::Runtime;

use super::{EncryptionMethod, Packet, PacketBytes};

impl Packet {
    #[instrument(skip(b))]
    pub fn from(mut b: BytesMut) -> Result<Self> {
        let mut p = Packet {
            flag: Flag::try_from_u32(b.get_u32())?,
            encryption_method: EncryptionMethod::try_from_u8(b.get_u8())?,
            ..Default::default()
        };
        b.get_u8(); // 0x00 固定

        p.uin = Packet::uin_from(&mut b)?;

        let mut payload = p.decrypt(b);
        let mut h = payload.get_4sized();

        p.sequence_number = h.get_u32();
        Packet::result_code_from(&mut h)?;
        Packet::error_msg_from(&mut h)?;
        p.cmd = h.get_4string();

        let app_sequence = h.get_4sized().get_i32();
        trace!(dsc = "app_sequence", seq = app_sequence);

        let compression_method = h.get_u32();
        let extra_data = h.get_4sized();
        trace!(dsc = "extra_data", dat = ?extra_data.to_vec());

        p.buffer = Packet::buffer_from(&mut payload, compression_method)?;

        Ok(p)
    }

    #[instrument(skip(b))]
    #[inline]
    fn uin_from(b: &mut BytesMut) -> Result<u64> {
        let a = b.get_4string();
        let r: Result<u64, _> = a.parse();
        match r {
            Ok(u) => Ok(u),
            Err(e) => {
                const DSC: &str = "uin 不是有效的类型";
                error!(dsc = DSC, uin = a, err = %e);
                bail!(DSC);
            }
        }
    }

    /// 解密
    #[inline]
    fn decrypt(&self, b: BytesMut) -> BytesMut {
        match self.encryption_method {
            EncryptionMethod::UnEncrypted => b,
            EncryptionMethod::D2Encrypted =>
                QTeaCipher::new(Runtime::get_d2key()).decrypt(b),
            EncryptionMethod::EmptyKeyEncrypted =>
                QTeaCipher::with_empty_key().decrypt(b),
        }
    }

    #[instrument(skip(b))]
    #[inline]
    fn result_code_from(b: &mut BytesMut) -> Result<()> {
        match b.get_i32() {
            0 => Ok(()),
            -10008 => bail!("会话失效"), // 可能需要指示上游刷新
            r => {
                const DSC: &str = "识别结果代码失败";
                error!(dsc = DSC, rec = r);
                bail!(DSC);
            }
        }
    }

    #[instrument(skip(b))]
    #[inline]
    fn error_msg_from(b: &mut BytesMut) -> Result<()> {
        let s = b.get_4string();
        if !s.is_empty() {
            error!(dsc = "上游服务器错误消息", msg = s);
            bail!(s);
        }

        Ok(())
    }

    #[instrument(skip(b, com))]
    #[inline]
    fn buffer_from(b: &mut BytesMut, com: u32) -> Result<BytesMut> {
        let p = b.get_4sized();
        match com {
            0 | 8 => Ok(p),
            1 => todo!(),
            _ => {
                const ERR: &str = "识别加密方式失败";
                error!(dsc = ERR, com = com);
                bail!(ERR);
            }
        }
    }
}
