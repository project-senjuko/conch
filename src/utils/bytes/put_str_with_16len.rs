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
    bytes::{BufMut, BytesMut},
    super::PutStrWith16Len,
};

impl PutStrWith16Len for BytesMut {
    fn put_string_with_16len(&mut self, s: &str) {
        let l = s.len();
        self.reserve(2 + l);
        self.put_u16(l as u16);
        self.extend(s.as_bytes());
    }
}
