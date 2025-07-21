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

pub struct DeletePlan{}
impl TryFrom<Message<'_>>
for DeletePlan {
    type Error = String;
    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        match msg.data_type() {
            Payload::DeletePlanResponse => {
                Ok(DeletePlan { })
            }
            e => Err(format!("Wrong datatype {:?}", e))
        }
    }
}

pub struct Plans (pub(crate) Vec<Plan>);


pub struct Plan {
    pub id: usize,
}

impl TryFrom<protocol::Plan<'_>> for Plan {
    type Error = String;

    fn try_from(initial: protocol::Plan) -> Result<Self, Self::Error> {
        let id = initial.id() as usize;
        Ok(Plan { id })
    }
}

impl TryFrom<Message<'_>> for Plans {
    type Error = String;

    fn try_from(msg: Message<'_>) -> Result<Self, Self::Error> {
        match msg.data_type() {
            Payload::Catalog => {
                msg.data_as_catalog().map(|c| {
                    let plans = match c.plans() {
                        Some(plans) => plans,
                        None => return Err(String::from("Contains no plans.")),
                    };
                    match Plans::try_from(plans) {
                        Ok(plan) => Ok(plan),
                        Err(err) => return Err(err)
                    }
                }).ok_or(String::from("Plans is empty"))?
            }
            e => Err(format!("Wrong datatype {:?}", e))
        }
    }
}

impl TryFrom<protocol::Plans<'_>> for Plans {
    type Error = String;

    fn try_from(initial: protocol::Plans<'_>) -> Result<Self, Self::Error> {
        let mut plans = vec![];
        for plan in initial.plans() {
            plans.push(Plan::try_from(plan)?)
        }
        Ok(Plans(plans))
    }
}

pub struct StartPlan {
    already_running: bool,
}

impl TryFrom<Message<'_>> for StartPlan {
    type Error = String;

    fn try_from(msg: Message<'_>) -> Result<Self, Self::Error> {
        match msg.data_type() {
            Payload::StartPlanResponse => {
                let already_running = msg.data_as_start_plan_response().ok_or(String::from("Did not contain start."))?.already_running();
                Ok(StartPlan { already_running })
            },
            err => Err(format!("Wrong datatype {:?}", err))
        }
    }
}

pub struct StopPlan {
    already_stopped: bool
}

impl TryFrom<Message<'_>> for StopPlan {
    type Error = String;
    fn try_from(msg: Message<'_>) -> Result<Self, Self::Error> {
        match msg.data_type() {
            Payload::StopPlanResponse => {
                let already_stopped = msg.data_as_stop_plan_response().ok_or(String::from("Did not contain start."))?.already_stopped();
                Ok(StopPlan { already_stopped })
            },
            err => Err(format!("Wrong datatype {:?}", err))
        }
    }
}