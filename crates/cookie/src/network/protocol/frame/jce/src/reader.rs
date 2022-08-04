////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

use bytes::{Buf, Bytes};
use rustc_hash::FxHashMap;

use crate::field::{HeadData, JceFieldErr, JceType};

pub struct JceReader<'a> {
    b: &'a mut Bytes,
    tag: u8,
    cache: FxHashMap<u8, (HeadData, Bytes)>,
}

impl<'a> JceReader<'a> {
    pub fn new(b: &'a mut Bytes) -> JceReader<'a> { JceReader { b, tag: 0, cache: FxHashMap::default() } }

    pub fn with_tag(b: &'a mut Bytes, tag: u8) -> JceReader<'a> { JceReader { b, tag, cache: FxHashMap::default() } }
}

impl<'a> JceReader<'a> {
    #[inline(always)]
    pub fn set_tag(&mut self, t: u8) { self.tag = t; }

    #[inline(always)]
    pub fn get<T: JceType<T>>(&mut self) -> Result<T, JceFieldErr> {
        match self.get_optional()? as Option<T> {
            Some(o) => Ok(o),
            None => Err(JceFieldErr { expectation: 255, result: 200 }),
        }
    }

    pub fn get_optional<T: JceType<T>>(&mut self) -> Result<Option<T>, JceFieldErr> {
        let r = self._get_optional();
        self.set_tag(self.tag + 1);
        r
    }

    #[inline(always)]
    fn _get_optional<T: JceType<T>>(&mut self) -> Result<Option<T>, JceFieldErr> {
        if !self.b.has_remaining() {
            return match self.cache.get(&self.tag).map(
                |o| T::from_bytes(&mut o.1.clone(), o.0.r#type),
            ) {
                None => { Ok(None) }
                Some(r) => {
                    match r {
                        Ok(t) => { Ok(Some(t)) }
                        Err(e) => { Err(e) }
                    }
                }
            };
        }

        let mut h = HeadData::parse(self.b);
        while h.tag != self.tag {
            let rb = self.b.clone();
            h.skip_value(self.b)?;
            self.cache.insert(
                h.tag,
                (h, rb.slice(..rb.len() - self.b.remaining())),
            );

            if self.b.has_remaining() {
                h = HeadData::parse(self.b);
            } else {
                return self._get_optional();
            }
        }

        Ok(Some(T::from_bytes(self.b, h.r#type)?))
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::field::{JByte, JceFieldErr, JString};

    use super::JceReader;

    #[test]
    fn get() -> Result<(), JceFieldErr> {
        let mut b = Bytes::from(
            vec![16, 1, 38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152],
        );
        let mut r = JceReader::with_tag(&mut b, 1);
        let num: JByte = r.get()?;
        let str: JString = r.get()?;

        assert_eq!(num, 1);
        assert_eq!(str, "千橘橘");
        Ok(())
    }

    #[test]
    fn get_wild() -> Result<(), JceFieldErr> {
        let mut b = Bytes::from(
            vec![38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152, 16, 1],
        );
        let mut r = JceReader::with_tag(&mut b, 1);
        let num: JByte = r.get()?;
        let str: JString = r.get()?;

        assert_eq!(num, 1);
        assert_eq!(str, "千橘橘");
        Ok(())
    }

    #[test]
    fn get_optional() -> Result<(), JceFieldErr> {
        let mut b = Bytes::from(
            vec![38, 9, 229, 141, 131, 230, 169, 152, 230, 169, 152],
        );
        let mut r = JceReader::with_tag(&mut b, 1);
        let num: Option<JByte> = r.get_optional()?;
        let str: JString = r.get()?;

        assert_eq!(num, None);
        assert_eq!(str, "千橘橘");
        Ok(())
    }
}
