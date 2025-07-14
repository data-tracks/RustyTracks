use crate::connection::Permission::AdminPermission;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Permission {
    AdminPermission,
}

impl TryFrom<&str> for Permission {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "admin" => Ok(AdminPermission),
            _ => Err(format!("unknown permission: {}", value)),
        }
    }
}