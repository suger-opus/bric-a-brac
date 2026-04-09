use crate::domain::{PropertiesDataModel, PropertyValueModel};
use bric_a_brac_dtos::{PropertiesDataDto, PropertyValueDto};

impl From<PropertyValueModel> for PropertyValueDto {
    fn from(model: PropertyValueModel) -> Self {
        match model {
            PropertyValueModel::String(value) => Self::String(value),
            PropertyValueModel::Number(value) => Self::Number(value),
            PropertyValueModel::Bool(value) => Self::Bool(value),
        }
    }
}

impl From<PropertyValueDto> for PropertyValueModel {
    fn from(dto: PropertyValueDto) -> Self {
        match dto {
            PropertyValueDto::String(value) => Self::String(value),
            PropertyValueDto::Number(value) => Self::Number(value),
            PropertyValueDto::Bool(value) => Self::Bool(value),
        }
    }
}

impl From<PropertiesDataModel> for PropertiesDataDto {
    fn from(model: PropertiesDataModel) -> Self {
        Self {
            values: model
                .values
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl From<PropertiesDataDto> for PropertiesDataModel {
    fn from(dto: PropertiesDataDto) -> Self {
        Self {
            values: dto.values.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}
