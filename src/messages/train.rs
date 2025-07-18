use crate::value::Value;
use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::Payload;

#[derive(Debug)]
pub struct Train {
    values: Vec<Value>
}

impl<'a> TryFrom<protocol::Message<'a>> for Train {
    type Error = String;

    fn try_from(msg: protocol::Message<'a>) -> Result<Self, Self::Error> {
        match msg.data_type() {
            Payload::Train => {
                match msg.data_as_train() {
                    None => Err(String::from("Train data type is not correct")),
                    Some(t) => {
                        let mut values = Vec::new();
                        for value in t.values() {
                            values.push(Value::from(value))
                        }
                        Ok(Train {values})
                    }
                }
            }
            _ => {
                Err(format!("Unexpected message type: {:?}", msg.data_type()))?
            }
        }
    }
}