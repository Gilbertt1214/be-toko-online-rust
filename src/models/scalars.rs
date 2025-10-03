use async_graphql::*;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

/// Custom scalar for Decimal type
#[derive(Clone, Debug)]
pub struct DecimalScalar(pub Decimal);

#[Scalar]
impl ScalarType for DecimalScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let decimal = s.parse::<Decimal>()
                    .map_err(|_| InputValueError::custom("Invalid decimal format"))?;
                Ok(DecimalScalar(decimal))
            }
            Value::Number(n) => {
                let s = n.to_string();
                let decimal = s.parse::<Decimal>()
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

/// Custom scalar for NaiveDateTime
#[derive(Clone, Debug)]
pub struct DateTimeScalar(pub NaiveDateTime);

#[Scalar]
impl ScalarType for DateTimeScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = value {
            let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| InputValueError::custom("Invalid datetime format. Use: YYYY-MM-DD HH:MM:SS"))?;
            Ok(DateTimeScalar(dt))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.format("%Y-%m-%d %H:%M:%S").to_string())
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