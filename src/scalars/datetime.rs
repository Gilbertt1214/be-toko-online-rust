use async_graphql::*;
use chrono::NaiveDateTime;

pub struct DateTimeScalar(pub NaiveDateTime);

#[Scalar]
impl ScalarType for DateTimeScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = &value {
            Ok(DateTimeScalar(
                NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f")
                    .map_err(|_| InputValueError::expected_type(value.clone()))?, 
            ))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.format("%Y-%m-%d %H:%M:%S%.f").to_string())
    }
}
