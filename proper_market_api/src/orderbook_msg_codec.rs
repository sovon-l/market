use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 35;
pub const SBE_TEMPLATE_ID: u16 = 3;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 1;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderbookMsgEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for OrderbookMsgEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for OrderbookMsgEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderbookMsgEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn symbol_encoder(self) -> SymbolEncoder<Self> {
            let offset = self.offset;
            SymbolEncoder::default().wrap(self, offset)
        }

        /// primitive field 'market_timestamp'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 18
        /// - encodedLength: 8
        #[inline]
        pub fn market_timestamp(&mut self, value: u64) {
            let offset = self.offset + 18;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'timestamp'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 26
        /// - encodedLength: 8
        #[inline]
        pub fn timestamp(&mut self, value: u64) {
            let offset = self.offset + 26;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        #[inline]
        pub fn orderbook_flags(&mut self, value: Orderbook_flags) {
            let offset = self.offset + 34;
            self.get_buf_mut().put_u8_at(offset, value.0)
        }

        /// GROUP ENCODER
        #[inline]
        pub fn depths_encoder(self, count: u8, depths_encoder: DepthsEncoder<Self>) -> DepthsEncoder<Self> {
            depths_encoder.wrap(self, count)
        }

    }

    #[derive(Debug, Default)]
    pub struct DepthsEncoder<P> {
        parent: Option<P>,
        count: u8,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for DepthsEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for DepthsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> DepthsEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u8,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 3);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u8_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            18
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// will return Some(current index) when successful otherwise None
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn price_encoder(self) -> DecEncoder<Self> {
            let offset = self.offset;
            DecEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn size_encoder(self) -> DecEncoder<Self> {
            let offset = self.offset + 9;
            DecEncoder::default().wrap(self, offset)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderbookMsgDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for OrderbookMsgDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for OrderbookMsgDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderbookMsgDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn symbol_decoder(self) -> SymbolDecoder<Self> {
            let offset = self.offset;
            SymbolDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn market_timestamp(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 18)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-1' }
        #[inline]
        pub fn timestamp(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 26);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn orderbook_flags(&self) -> Orderbook_flags {
            Orderbook_flags::new(self.get_buf().get_u8_at(self.offset + 34))
        }

        /// GROUP DECODER
        #[inline]
        pub fn depths_decoder(self) -> DepthsDecoder<Self> {
            let acting_version = self.acting_version;
            DepthsDecoder::default().wrap(self, acting_version as usize)
        }

    }

    #[derive(Debug, Default)]
    pub struct DepthsDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u8,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for DepthsDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for DepthsDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> DepthsDecoder<P> where P: Decoder<'a> + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
            acting_version: usize,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset) as usize;
            let count = parent.get_buf().get_u8_at(initial_offset + 2);
            parent.set_limit(initial_offset + 3);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.acting_version = acting_version;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        /// group token - Token{signal=BEGIN_GROUP, name='depths', referencedName='null', description='null', id=5, version=0, deprecated=0, encodedLength=18, offset=35, componentTokenCount=18, encoding=Encoding{presence=REQUIRED, primitiveType=null, byteOrder=LITTLE_ENDIAN, minValue=null, maxValue=null, nullValue=null, constValue=null, characterEncoding='null', epoch='null', timeUnit=null, semanticType='null'}}
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn count(&self) -> u8 {
            self.count
        }

        /// will return Some(current index) when successful otherwise None
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn price_decoder(self) -> DecDecoder<Self> {
            let offset = self.offset;
            DecDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn size_decoder(self) -> DecDecoder<Self> {
            let offset = self.offset + 9;
            DecDecoder::default().wrap(self, offset)
        }

    }

} // end decoder

