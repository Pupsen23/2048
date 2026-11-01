use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidProbability {
        probability: Decimal,
        min: Decimal,
        max: Decimal,
    },
}
