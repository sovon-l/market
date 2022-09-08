#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderbookError {
    DeleteNonexistDepth,
    InvalidUpdate,
}

impl std::fmt::Display for OrderbookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderbookError::DeleteNonexistDepth => write!(f, "DeleteNonexistDepth"),
            OrderbookError::InvalidUpdate => write!(f, "InvalidUpdate"),
        }
    }
}
impl std::error::Error for OrderbookError {}
