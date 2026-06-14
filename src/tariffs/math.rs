use fixed::types::I64F64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityAmount {
    pub integral: i64,
    pub fractional: u64,
}

pub fn scale_commodity(amount: f64, _decimals: u8) -> Result<I64F64, &'static str> {
    let scaled = I64F64::from_num(amount);
    Ok(scaled)
}

pub fn convert_units(
    value: I64F64,
    from_unit: &str,
    to_unit: &str,
) -> Result<I64F64, &'static str> {
    match (from_unit, to_unit) {
        ("kWh", "MWh") => Ok(value / I64F64::from_num(1000)),
        ("gal", "CCF") => Ok(value / I64F64::from_num(748)),
        ("m3", "L") => Ok(value * I64F64::from_num(1000)),
        _ => Err("unsupported unit conversion pair"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn no_panic_on_extreme_values(v in -1e12f64..1e12f64) {
            let _ = scale_commodity(v, 7);
        }
    }
}
