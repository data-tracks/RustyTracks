use crate::connection::Permission::Admin;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Permission {
    Admin,
}

impl TryFrom<&str> for Permission {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "admin" => Ok(Admin),
            _ => Err(format!("unknown permission: {}", value)),
        }
    }
}