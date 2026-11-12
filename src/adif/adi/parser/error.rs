use thiserror::Error;

use crate::adif::LengthUnit;

#[derive(Debug, Error)]
pub enum AdiError {
    #[error(
        "truncated value for {field_name}: declared {declared}, \
     found {available} before EOF (unit: {unit:?})"
    )]
    TruncatedValue {
        field_name: String,
        declared: usize,
        available: usize,
        unit: LengthUnit,
    },
}
