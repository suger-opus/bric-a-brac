use crate::DtosConversionError;
use bric_a_brac_id::id;
use std::str::FromStr;

id!(UserIdDto);

impl TryFrom<String> for UserIdDto {
    type Error = DtosConversionError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(&s)?)
    }
}
