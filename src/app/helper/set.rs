#![allow(dead_code)]

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;

pub fn empty_string_to_option_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub fn deserialize_string_to_bigdecimal<'de, D>(
    deserializer: D,
) -> Result<Option<BigDecimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_str = Option::<String>::deserialize(deserializer)?;

    if let Some(s) = opt_str {
        let cleaned: String = s
            .replace(",", "")
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
            .collect();

        if cleaned.trim().is_empty() {
            return Ok(None);
        }

        let parsed = BigDecimal::from_str(&cleaned).unwrap_or_else(|_| BigDecimal::zero());
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}
