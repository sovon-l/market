#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Orderbook_flags(pub u8);
impl Orderbook_flags {
    #[inline]
    pub fn new(value: u8) -> Self {
        Orderbook_flags(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_is_snapshot(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_is_snapshot(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }
}
impl core::fmt::Debug for Orderbook_flags {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "Orderbook_flags[is_snapshot(0)={}]",
            self.get_is_snapshot(),)
    }
}
