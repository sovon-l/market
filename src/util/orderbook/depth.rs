
// IMPORTANT: using convention if price > midprice then size is -ve
#[derive(Clone, Copy)]
pub struct MbpDepth {
    pub price: rust_decimal::Decimal,
    pub size: rust_decimal::Decimal,
}

impl Ord for MbpDepth {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}
impl PartialOrd for MbpDepth {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for MbpDepth {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}
impl Eq for MbpDepth {}
impl std::fmt::Display for MbpDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.price, self.size)
    }
}
impl std::fmt::Debug for MbpDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.price, self.size)
    }
}
