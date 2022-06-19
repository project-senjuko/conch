////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{BufMut, BytesMut};

use super::field::JceType;

pub struct JceWriter {
    b: BytesMut,
    tag: u8,
}

impl JceWriter {
    pub fn new(tag: u8) -> JceWriter { JceWriter { b: BytesMut::new(), tag } }
}

impl JceWriter {
    pub fn set_tag(&mut self, t: u8) { self.tag = t; }

    pub fn put<T: JceType<T>>(&mut self, t: &T) {
        t.to_bytes(&mut self.b, self.tag);
        self.set_tag(self.tag + 1);
    }

    pub fn to_bytes(self, b: &mut BytesMut) { b.put(self.b); }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::JceWriter;

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        let mut w = JceWriter::new(1);
        w.put(&1);
        w.put(&String::from("千橘橘"));
        w.to_bytes(&mut b);
        assert_eq!(b.to_vec(), vec![16, 1, 38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152]);
    }
}
