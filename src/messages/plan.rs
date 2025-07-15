use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::{Message, Payload};

pub struct CreatePlan{
    pub id: usize
}

impl TryFrom<Message<'_>> for CreatePlan {
    type Error = String;

    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        match msg.data_type() {
            Payload::CreatePlanResponse => {
                Ok(CreatePlan { id: msg.data_as_create_plan_response().ok_or(String::from("Did not contain id."))?.id() as usize })
            },
            e => Err(format!("Wrong datatype {:?}", e))
        }

    }
}