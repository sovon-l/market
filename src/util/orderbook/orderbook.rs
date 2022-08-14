
pub trait Orderbook {
    fn get_mut_book(&mut self) -> &mut Vec<crate::util::orderbook::depth::MbpDepth>;
    
    fn update_book(&mut self, updates: &[crate::util::orderbook::depth::MbpDepth]) -> Result<(), crate::util::orderbook::error::OrderbookError> {
        let mut delete_nonexist_depth = false;
        for depth in updates {
            let self_book = self.get_mut_book();
            match update_page(self_book, *depth) {
                Ok(_) => (),
                Err(_) => delete_nonexist_depth = true,
            }
        }
        if delete_nonexist_depth {
            Err(crate::util::orderbook::error::OrderbookError::DeleteNonexistDepth)
        } else {
            Ok(())
        }
    }
}

fn update_page(page: &mut Vec<crate::util::orderbook::depth::MbpDepth>, depth: crate::util::orderbook::depth::MbpDepth) -> Result<(), crate::util::orderbook::error::OrderbookError> {
    match page.binary_search(&depth) {
        Ok(i) => {
            if depth.size.is_zero() {
                page.remove(i);
            } else {
                let x = page.get_mut(i).unwrap();
                *x = depth;
            }
        }
        Err(i) => {
            if depth.size.is_zero() {
                return Err(crate::util::orderbook::error::OrderbookError::DeleteNonexistDepth)
            } else {
                page.insert(i, depth);
            }
        }
    }
    Ok(())
}