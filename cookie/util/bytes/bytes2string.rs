////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BytesMut};
use tracing::{error, instrument};

use super::Bytes2String;

impl Bytes2String for BytesMut {
    #[instrument(skip(self))]
    fn get_string(&mut self, len: usize) -> String {
        let s = String::from_utf8(self[..len].to_owned());
        if s.is_err() {
            error!(dsc = "解析为 utf8 字符串失败", err = %s.as_ref().unwrap_err());
        }

        self.advance(len);
        s.unwrap_or_default()
    }
}
