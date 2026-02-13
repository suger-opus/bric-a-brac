use crate::{
    dtos::RoleDto,
    models::{GraphId, NewAccess, UserId},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateAccessRequest {
    pub user_id: UserId,
    pub role: RoleDto,
}

impl CreateAccessRequest {
    pub fn into_domain(self, graph_id: GraphId) -> NewAccess {
        NewAccess {
            graph_id,
            user_id: self.user_id,
            role: self.role.into(),
        }
    }
}
