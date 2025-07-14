use crate::connection::Permission;
use track_rails::message_generated::protocol::{Message, Payload};

#[derive(Debug)]
pub struct RegisterResponse {
    pub(crate) permissions: Vec<Permission>,
    id: usize
}

impl TryFrom<Message<'_>> for RegisterResponse {
    type Error = String;

    fn try_from(msg: Message<'_>) -> Result<Self, Self::Error> {
        match msg.data_type() {
            Payload::RegisterResponse => {
                let permissions = msg.data_as_register_response().unwrap().permissions().unwrap_or_default();
                let permissions = permissions.into_iter().map(|buffer: &str| {
                    Permission::try_from(buffer)
                }).filter_map(|s|s.ok()).collect();
                let id = msg.data_as_register_response().unwrap().id().map(|id| id as usize).ok_or("Could not get id")?;
                Ok(RegisterResponse{permissions, id })
            }
            _ => Err(String::from("Could not transform"))
        }

    }
}