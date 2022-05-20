use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 18;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct InstrumentEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for InstrumentEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> InstrumentEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// REQUIRED enum
        #[inline]
        pub fn exchange(&mut self, value: Exchange) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'quote'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn quote(&mut self, value: [u8; 6]) {
            let offset = self.offset + 1;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
        }

        /// primitive array field 'base'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 7
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn base(&mut self, value: [u8; 6]) {
            let offset = self.offset + 7;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
        }

        /// REQUIRED enum
        #[inline]
        pub fn instrument_type(&mut self, value: InstrumentType) {
            let offset = self.offset + 13;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'expiry'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 14
        /// - encodedLength: 4
        #[inline]
        pub fn expiry(&mut self, value: u32) {
            let offset = self.offset + 14;
            self.get_buf_mut().put_u32_at(offset, value);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct InstrumentDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for InstrumentDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> InstrumentDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// REQUIRED enum
        #[inline]
        pub fn exchange(&self) -> Exchange {
            self.get_buf().get_u8_at(self.offset).into()
        }

        #[inline]
        pub fn quote(&self) -> [u8; 6] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 1),
                buf.get_u8_at(self.offset + 1 + 1),
                buf.get_u8_at(self.offset + 1 + 2),
                buf.get_u8_at(self.offset + 1 + 3),
                buf.get_u8_at(self.offset + 1 + 4),
                buf.get_u8_at(self.offset + 1 + 5),
            ]
        }

        #[inline]
        pub fn base(&self) -> [u8; 6] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 7),
                buf.get_u8_at(self.offset + 7 + 1),
                buf.get_u8_at(self.offset + 7 + 2),
                buf.get_u8_at(self.offset + 7 + 3),
                buf.get_u8_at(self.offset + 7 + 4),
                buf.get_u8_at(self.offset + 7 + 5),
            ]
        }

        /// REQUIRED enum
        #[inline]
        pub fn instrument_type(&self) -> InstrumentType {
            self.get_buf().get_u8_at(self.offset + 13).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '4294967295' }
        #[inline]
        pub fn expiry(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 14);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

    }
} // end decoder mod 
