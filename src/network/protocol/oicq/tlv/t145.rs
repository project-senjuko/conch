////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use {
    bytes::BytesMut,
    crate::runtime::Runtime,
    super::TlvField,
};

struct TlvT145 {
    guid: [u8; 16],
}

impl Default for TlvT145 {
    fn default() -> Self {
        Self {
            guid: Runtime::secret().guid,
        }
    }
}

impl TlvField for TlvT145 {
    fn tag() -> u16 { 0x145 }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(16);
        b.extend_from_slice(&self.guid);
    }
}
