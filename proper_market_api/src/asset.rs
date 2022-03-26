#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Asset {
    usd = 0x0_u8, 
    btc = 0x1_u8, 
    eth = 0x2_u8, 
    usdt = 0x3_u8, 
    NullVal = 0xff_u8, 
}
impl Default for Asset {
    #[inline]
    fn default() -> Self { Asset::NullVal }
}
impl From<u8> for Asset {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::usd, 
            0x1_u8 => Self::btc, 
            0x2_u8 => Self::eth, 
            0x3_u8 => Self::usdt, 
            _ => Self::NullVal,
        }
    }
}
