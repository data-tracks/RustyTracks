use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::Payload;
use crate::connection::Permission;

#[derive(Debug)]
pub struct Message{
    permissions: Vec<Permission>,
}

impl Message {
    pub fn new(permissions: Vec<Permission>) -> Message {
        Message{permissions}
    }
}

impl From<protocol::Message<'_>> for Message {
    fn from(msg: protocol::Message<'_>) -> Self {
        let permissions = match msg.data_type() {
            Payload::RegisterResponse => {
                let permission = msg.data_as_register_response().unwrap().permissions().unwrap_or_default();
                permission.into_iter().map(|buffer: &str| {
                   Permission::try_from(buffer)
                }).filter_map(|s|s.ok()).collect()
            }
            _ => vec![]
        };
        Message::new(permissions)
    }
}