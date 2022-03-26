pub fn split_currency_quote<'a>(
    symbol: &'a str,
    quotes: &[&'static str],
) -> Option<(&'a str, &'a str)> {
    for quote in quotes {
        let temp = &symbol[symbol.len() - quote.len()..symbol.len()];
        if temp == *quote {
            return Some((&symbol[..symbol.len() - quote.len()], quote));
        }
    }
    None
}
