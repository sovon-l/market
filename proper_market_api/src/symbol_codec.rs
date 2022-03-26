use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 8;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SymbolEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for SymbolEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> SymbolEncoder<P> where P: Writer<'a> + Default {
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

        /// REQUIRED enum
        #[inline]
        pub fn quote(&mut self, value: Asset) {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn base(&mut self, value: Asset) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn symbol_type(&mut self, value: SymbolType) {
            let offset = self.offset + 3;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'expiry'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 4
        /// - encodedLength: 4
        #[inline]
        pub fn expiry(&mut self, value: u32) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u32_at(offset, value);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SymbolDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for SymbolDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> SymbolDecoder<P> where P: Reader<'a> + Default {
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

        /// REQUIRED enum
        #[inline]
        pub fn quote(&self) -> Asset {
            self.get_buf().get_u8_at(self.offset + 1).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn base(&self) -> Asset {
            self.get_buf().get_u8_at(self.offset + 2).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn symbol_type(&self) -> SymbolType {
            self.get_buf().get_u8_at(self.offset + 3).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '4294967295' }
        #[inline]
        pub fn expiry(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 4);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

    }
} // end decoder mod 
