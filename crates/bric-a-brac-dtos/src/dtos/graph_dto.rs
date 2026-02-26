use crate::DtosConversionError;
use bric_a_brac_id::id;
use std::str::FromStr;

id!(GraphIdDto);

impl TryFrom<String> for GraphIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(GraphIdDto::from_str(&s)?)
    }
}
