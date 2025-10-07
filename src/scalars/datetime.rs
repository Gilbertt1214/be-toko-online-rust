use async_graphql::*;
use chrono::{NaiveDateTime, DateTime, Utc};
use rust_decimal::Decimal;

/// Scalar untuk Decimal (harga, angka uang)
#[derive(Clone, Debug)]
pub struct DecimalScalar(pub Decimal);

#[Scalar]
impl ScalarType for DecimalScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let decimal = s
                    .parse::<Decimal>()
                    .map_err(|_| InputValueError::custom("Invalid decimal format"))?;
                Ok(DecimalScalar(decimal))
            }
            Value::Number(n) => {
                let s = n.to_string();
                let decimal = s
                    .parse::<Decimal>()
                    .map_err(|_| InputValueError::custom("Invalid decimal format"))?;
                Ok(DecimalScalar(decimal))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

impl From<Decimal> for DecimalScalar {
    fn from(d: Decimal) -> Self {
        DecimalScalar(d)
    }
}

impl From<DecimalScalar> for Decimal {
    fn from(ds: DecimalScalar) -> Self {
        ds.0
    }
}

/// ================================
/// Scalar untuk DateTime (format ISO 8601)
/// ================================
#[derive(Clone, Debug)]
pub struct DateTimeScalar(pub NaiveDateTime);

#[Scalar]
impl ScalarType for DateTimeScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = value {
            // Parsing dari format ISO 8601
            let dt = DateTime::parse_from_rfc3339(&s)
                .map_err(|_| InputValueError::custom("Invalid datetime format. Use ISO 8601 (e.g., 2025-10-04T22:30:00Z)"))?
                .with_timezone(&Utc)
                .naive_utc();

            Ok(DateTimeScalar(dt))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        // Ubah ke string ISO 8601
        let datetime = DateTime::<Utc>::from_utc(self.0, Utc);
        Value::String(datetime.to_rfc3339())
    }
}

impl From<NaiveDateTime> for DateTimeScalar {
    fn from(dt: NaiveDateTime) -> Self {
        DateTimeScalar(dt)
    }
}

impl From<DateTimeScalar> for NaiveDateTime {
    fn from(dts: DateTimeScalar) -> Self {
        dts.0
    }
}
