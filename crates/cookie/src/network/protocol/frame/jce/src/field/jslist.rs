////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, BufMut, Bytes, BytesMut};

use super::{BYTE, HeadData, JceFieldErr, JceType, JSList, SIMPLE_LIST};

impl JceType<JSList> for JSList {
    fn to_bytes(&self, b: &mut BytesMut, tag: u8) {
        HeadData::new(SIMPLE_LIST, tag).format(b, self.remaining());
        HeadData::new(BYTE, 0).format(b, 0);
        (self.remaining() as i32).to_bytes(b, 0);
        b.put(self.slice(..));
    }

    fn from_bytes(b: &mut Bytes, _: u8) -> Result<JSList, JceFieldErr> {
        {
            let head = HeadData::parse(b);
            if head.tag != 0 || head.r#type != 0 {
                return Err(JceFieldErr { expectation: 0, result: head.r#type });
            }
        }
        let len = HeadData::parse_ttl4(b)?;
        let a = b.slice(..len);
        b.advance(len);
        Ok(a)
    }
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::{JceType, JSList, SIMPLE_LIST};

    #[test]
    fn to_bytes() {
        let mut b = BytesMut::new();
        Bytes::from(vec![1, 1, 4, 5, 1, 4]).to_bytes(&mut b, 0);
        assert_eq!(b.to_vec(), vec![13, 0, 0, 6, 1, 1, 4, 5, 1, 4]);
    }

    #[test]
    fn from_bytes() {
        assert_eq!(
            JSList::from_bytes(
                &mut Bytes::from(vec![0, 0, 6, 1, 1, 4, 5, 1, 4]),
                SIMPLE_LIST,
            ).unwrap(),
            Bytes::from(vec![1, 1, 4, 5, 1, 4]),
        );
    }
}
