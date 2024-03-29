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

use super::GetSized;

impl GetSized for BytesMut {
    fn get_sized(&mut self, len: usize) -> Self {
        let mut b = BytesMut::zeroed(len);
        b.swap_with_slice(&mut self[..len]);
        self.advance(len);

        b
    }
}

impl GetSized for Bytes {
    fn get_sized(&mut self, len: usize) -> Self {
        let b = self.slice(..len);
        self.advance(len);

        b
    }
}
