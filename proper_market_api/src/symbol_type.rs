#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SymbolType {
    spot = 0x0_u8, 
    future = 0x1_u8, 
    inverse_future = 0x2_u8, 
    NullVal = 0xff_u8, 
}
impl Default for SymbolType {
    #[inline]
    fn default() -> Self { SymbolType::NullVal }
}
impl From<u8> for SymbolType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::spot, 
            0x1_u8 => Self::future, 
            0x2_u8 => Self::inverse_future, 
            _ => Self::NullVal,
        }
    }
}
