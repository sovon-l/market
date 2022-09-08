use crate::util::orderbook::depth::MbpDepth;

pub trait VerificationStatus {
    type VerificationUpdate;
    fn verify(
        orderbook: &[crate::util::orderbook::depth::MbpDepth],
        status: &mut Self,
        update: &mut Self::VerificationUpdate,
    ) -> Option<bool>;
    // None: do nothing
    // Some(true): update
    // Some(false): fail verification - need to re-request snapshot
}

#[derive(Debug, Clone)]
pub struct MbpUpdate<VUpdate> {
    pub depths: Vec<MbpDepth>,
    pub verification_update: VUpdate,
}

#[derive(Debug, Clone)]
pub struct MbpFullbook<VStatus> {
    pub depths: Vec<MbpDepth>,
    pub verification_status: VStatus,
}

impl<V> crate::util::orderbook::orderbook::Orderbook for MbpFullbook<V> {
    fn get_mut_book(&mut self) -> &mut Vec<crate::util::orderbook::depth::MbpDepth> {
        &mut self.depths
    }
}

impl<VUpdate, VStatus: VerificationStatus<VerificationUpdate = VUpdate>> MbpFullbook<VStatus> {
    pub fn update(
        &mut self,
        mut update: MbpUpdate<VUpdate>,
    ) -> Result<(), crate::util::orderbook::error::OrderbookError> {
        crate::util::orderbook::orderbook::Orderbook::update_book(self, &update.depths)?;
        if let Some(no_need_snapshot) = VStatus::verify(
            &self.depths,
            &mut self.verification_status,
            &mut update.verification_update,
        ) {
            if !no_need_snapshot {
                return Err(crate::util::orderbook::error::OrderbookError::InvalidUpdate);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::orderbook::depth::MbpDepth;
    use crate::util::orderbook::error::OrderbookError;
    use rust_decimal_macros::dec;
    use std::error::Error;

    mod seqnum_onebyone {
        use super::super::*;
        pub struct SeqNum {
            pub seqnum: u64,
        }
        impl VerificationStatus for SeqNum {
            type VerificationUpdate = SeqNum;
            fn verify(
                _orderbook: &[crate::util::orderbook::depth::MbpDepth],
                status: &mut Self,
                update: &mut Self::VerificationUpdate,
            ) -> Option<bool> {
                if status.seqnum + 1 == update.seqnum {
                    status.seqnum = update.seqnum;
                    Some(true)
                } else {
                    Some(false)
                }
            }
        }
    }

    // binance special case
    mod seqnum_bunch {
        use super::super::*;
        pub struct SeqNum {
            pub seqnum: u64,
        }
        impl VerificationStatus for SeqNum {
            type VerificationUpdate = (u64, u64);
            fn verify(
                _orderbook: &[crate::util::orderbook::depth::MbpDepth],
                status: &mut Self,
                update: &mut Self::VerificationUpdate,
            ) -> Option<bool> {
                if status.seqnum < update.0 + 1 {
                    return Some(false);
                }
                if status.seqnum < update.1 {
                    return None;
                }
                status.seqnum = update.1;
                Some(true)
            }
        }
    }

    mod checksum {
        use super::super::*;
        pub struct CheckSum {
            pub hash: u64,
        }
        pub fn hashing(depths: &[crate::util::orderbook::depth::MbpDepth]) -> u64 {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let hash_content = depths
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("");
            let mut hasher = DefaultHasher::new();
            hash_content.hash(&mut hasher);
            hasher.finish()
        }
        impl VerificationStatus for CheckSum {
            type VerificationUpdate = Self;
            fn verify(
                orderbook: &[crate::util::orderbook::depth::MbpDepth],
                _status: &mut Self,
                update: &mut Self::VerificationUpdate,
            ) -> Option<bool> {
                Some(update.hash == hashing(orderbook))
            }
        }
    }

    #[test]
    fn test_seqnum_onebyone_success() -> Result<(), Box<dyn Error>> {
        let mut book = MbpFullbook {
            depths: vec![
                MbpDepth {
                    price: dec!(1.0),
                    size: dec!(1.0),
                },
                MbpDepth {
                    price: dec!(2.0),
                    size: dec!(1.0),
                },
                MbpDepth {
                    price: dec!(3.0),
                    size: dec!(0.5),
                },
                MbpDepth {
                    price: dec!(4.0),
                    size: dec!(-1.0),
                },
                MbpDepth {
                    price: dec!(5.0),
                    size: dec!(-1.0),
                },
            ],
            verification_status: seqnum_onebyone::SeqNum { seqnum: 0 },
        };
        let update = MbpUpdate {
            depths: vec![MbpDepth {
                price: dec!(1.0),
                size: dec!(2.0),
            }],
            verification_update: seqnum_onebyone::SeqNum { seqnum: 1 },
        };
        book.update(update)?;
        assert_eq!(book.depths[0].size, dec!(2.0));
        Ok(())
    }

    #[test]
    fn test_seqnum_onebyone_jump() -> Result<(), Box<dyn Error>> {
        let mut book = MbpFullbook {
            depths: vec![
                MbpDepth {
                    price: dec!(1.0),
                    size: dec!(1.0),
                },
                MbpDepth {
                    price: dec!(2.0),
                    size: dec!(1.0),
                },
                MbpDepth {
                    price: dec!(3.0),
                    size: dec!(0.5),
                },
                MbpDepth {
                    price: dec!(4.0),
                    size: dec!(-1.0),
                },
                MbpDepth {
                    price: dec!(5.0),
                    size: dec!(-1.0),
                },
            ],
            verification_status: seqnum_onebyone::SeqNum { seqnum: 0 },
        };
        let update = MbpUpdate {
            depths: vec![MbpDepth {
                price: dec!(1.0),
                size: dec!(2.0),
            }],
            verification_update: seqnum_onebyone::SeqNum { seqnum: 2 },
        };
        let rt = book.update(update);
        // assert!(rt.is_err());
        // let rt = rt.unwrap_err();
        assert!(matches!(rt, Err(OrderbookError::InvalidUpdate)));
        Ok(())
    }

    #[test]
    fn test_checksum() -> Result<(), Box<dyn Error>> {
        let mut book = MbpFullbook {
            depths: vec![
                MbpDepth {
                    price: dec!(1.0),
                    size: dec!(1.0),
                },
                MbpDepth {
                    price: dec!(2.0),
                    size: dec!(1.0),
                },
                MbpDepth {
                    price: dec!(3.0),
                    size: dec!(0.5),
                },
                MbpDepth {
                    price: dec!(4.0),
                    size: dec!(-1.0),
                },
                MbpDepth {
                    price: dec!(5.0),
                    size: dec!(-1.0),
                },
            ],
            verification_status: checksum::CheckSum { hash: 0 },
        };
        assert_eq!(checksum::hashing(&book.depths), 10157360891676294958);
        let update = MbpUpdate {
            depths: vec![MbpDepth {
                price: dec!(1.0),
                size: dec!(2.0),
            }],
            verification_update: checksum::CheckSum {
                hash: 15655222248316742011,
            },
        };
        let rt = book.update(update);
        assert!(rt.is_ok());
        Ok(())
    }
}
