use crate::domain::models::PropertiesDataModel;
use bric_a_brac_dtos::PropertiesDataDto;

impl From<PropertiesDataModel> for PropertiesDataDto {
    fn from(model: PropertiesDataModel) -> Self {
        PropertiesDataDto {
            values: model.values,
        }
    }
}

impl From<PropertiesDataDto> for PropertiesDataModel {
    fn from(dto: PropertiesDataDto) -> Self {
        PropertiesDataModel { values: dto.values }
    }
}
