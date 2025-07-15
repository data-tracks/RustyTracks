use std::ops::{Deref, DerefMut};
use crate::connection::Connection;
use crate::messages::CreatePlan;
use flatbuffers::FlatBufferBuilder;
use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::{CreatePlanRequest, CreatePlanRequestArgs, MessageArgs, OkStatus, OkStatusArgs, Payload, Status};

pub struct Admin{
    connection: Connection,
}


impl Admin {
    pub(crate) fn new(connection: Connection) -> Self {
        Admin{ connection }
    }

    pub(crate) fn create_plan<Name: AsRef<str>, Plan: AsRef<str>>(&mut self, name: Name, plan: Plan) -> Result<usize, String> {
        let mut builder = FlatBufferBuilder::new();

        let name = builder.create_string(name.as_ref());
        let plan = builder.create_string(plan.as_ref());

        let create = CreatePlanRequest::create(&mut builder, &CreatePlanRequestArgs{ name: Some(name), plan: Some(plan) }).as_union_value();

        let status = OkStatus::create(&mut builder, &OkStatusArgs { }).as_union_value();

        let msg = protocol::Message::create(&mut builder, &MessageArgs{
            data_type: Payload::CreatePlanRequest,
            data: Some(create),
            status_type: Status::OkStatus,
            status: Some(status),
        });

        builder.finish(msg, None);
        let msg = builder.finished_data();

        self.connection.write_all(msg)?;

        let res: CreatePlan = self.connection.read()?;

        Ok(res.id)
    }
}

impl Deref for Admin {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl DerefMut for Admin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.connection
    }
}
