use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::Payload;
use crate::connection::Permission;

#[derive(Debug)]
pub struct Message{
    pub permissions: Vec<Permission>,
    pub id: Option<usize>
}

impl Message {
    pub fn new(permissions: Vec<Permission>, id: Option<usize>) -> Message {
        Message{permissions, id }
    }
}

impl<'a> TryFrom<protocol::Message<'a>> for Message {
    type Error = String;

    fn try_from(msg: protocol::Message<'a>) -> Result<Self, Self::Error> {
        let permissions = match msg.data_type() {
            Payload::RegisterResponse => {
                let permission = msg.data_as_register_response().unwrap().permissions().unwrap_or_default();
                permission.into_iter().map(|buffer: &str| {
                    Permission::try_from(buffer)
                }).filter_map(|s|s.ok()).collect()
            }
            _ => vec![]
        };

        let id = match msg.data_type() {
            Payload::RegisterResponse => {
                msg.data_as_register_response().unwrap().id().map(|id| id as usize)
            }
            _ => None
        };

        Ok(Message::new(permissions, id))
    }
}