////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.moe>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
//   More information at https://github.com/project-senjuko/conch.                                 /
////////////////////////////////////////////////////////////////////////////////////////////////////

use {
    super::TlvField,
    crate::{runtime::Runtime, utils::bytes::PutStrWith16Len},
    bytes::{BufMut, BytesMut},
};

pub struct TlvT128 {
    pub guid_from_file_null: u8,
    pub generate_guid_null: u8,
    pub generate_guid_equal_guid_from_file: u8,
    pub wtf: u32,
}

impl Default for TlvT128 {
    fn default() -> Self {
        Self {
            guid_from_file_null: 1,
            generate_guid_null: 1,
            generate_guid_equal_guid_from_file: 0,
            wtf: 285212672,
        }
    }
}

impl TlvField for TlvT128 {
    fn tag(&self) -> u16 {
        0x128
    }

    fn to_payload(&self, b: &mut BytesMut) {
        b.reserve(35);
        b.put_u16(0);
        b.put_u8(self.guid_from_file_null);
        b.put_u8(self.generate_guid_null);
        b.put_u8(self.generate_guid_equal_guid_from_file);
        b.put_u32(self.wtf);
        b.put_string_with_16len("Conch"); // const need
        b.put_u16(10);
        b.extend(Runtime::secret().guid); // pref need
        b.put_string_with_16len("Conch"); // const need
    }
}
