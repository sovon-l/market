#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Exchange {
    binance = 0x0_u8, 
    ftx = 0x1_u8, 
    NullVal = 0xff_u8, 
}
impl Default for Exchange {
    #[inline]
    fn default() -> Self { Exchange::NullVal }
}
impl From<u8> for Exchange {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::binance, 
            0x1_u8 => Self::ftx, 
            _ => Self::NullVal,
        }
    }
}
