use crate::DtosConversionError;
use bric_a_brac_protos::common::RoleProto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, derive_more::Display)]
pub enum RoleDto {
    #[display("owner")]
    Owner,
    #[display("admin")]
    Admin,
    #[display("editor")]
    Editor,
    #[display("viewer")]
    Viewer,
    #[display("none")]
    None,
}

impl From<RoleProto> for RoleDto {
    fn from(value: RoleProto) -> Self {
        match value {
            RoleProto::RoleOwner => Self::Owner,
            RoleProto::RoleAdmin => Self::Admin,
            RoleProto::RoleEditor => Self::Editor,
            RoleProto::RoleViewer => Self::Viewer,
            RoleProto::RoleNone => Self::None,
        }
    }
}

impl From<RoleDto> for RoleProto {
    fn from(value: RoleDto) -> Self {
        match value {
            RoleDto::Owner => Self::RoleOwner,
            RoleDto::Admin => Self::RoleAdmin,
            RoleDto::Editor => Self::RoleEditor,
            RoleDto::Viewer => Self::RoleViewer,
            RoleDto::None => Self::RoleNone,
        }
    }
}

impl TryFrom<i32> for RoleDto {
    type Error = DtosConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        #[allow(clippy::map_err_ignore)]
        RoleProto::try_from(value)
            .map(Self::from)
            .map_err(|_| Self::Error::UnknownEnumVariant {
                enum_name: "RoleProto".to_owned(),
                value,
            })
    }
}

impl From<RoleDto> for i32 {
    fn from(dto: RoleDto) -> Self {
        Self::from(RoleProto::from(dto))
    }
}
