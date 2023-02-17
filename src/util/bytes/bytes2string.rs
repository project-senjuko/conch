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

use bytes::{Buf, Bytes, BytesMut};

use super::{Bytes2String, GetSized};

impl Bytes2String for BytesMut {
    //noinspection DuplicatedCode
    fn get_string(&mut self, len: usize) -> String {
        let b = self.get_sized(len);
        String::from_utf8_lossy(b.chunk()).to_string()
    }
}

impl Bytes2String for Bytes {
    //noinspection DuplicatedCode
    fn get_string(&mut self, len: usize) -> String {
        let b = self.get_sized(len);
        String::from_utf8_lossy(b.chunk()).to_string()
    }
}
