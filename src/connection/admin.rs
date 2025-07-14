use flatbuffers::FlatBufferBuilder;
use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::{Create, CreateArgs, CreatePlanRequest, CreatePlanRequestArgs, CreateType, MessageArgs, OkStatus, OkStatusArgs, Payload, Status};
use crate::Client;
use crate::connection::{client, Connection};

pub struct Admin<'c>{
    connection: &'c Connection,
}


impl<'c> Admin<'c> {
    pub(crate) fn new(connection: &'c Connection) -> Self {
        Admin{ connection }
    }

    pub(crate) fn create_plan<Name: AsRef<str>, Plan: AsRef<str>>(&mut self, name: Name, plan: Plan) -> Result<usize, String> {
        let mut builder = FlatBufferBuilder::new();
        
        let name = builder.create_string(name.as_ref());
        let plan = builder.create_string(plan.as_ref());
        
        let create = CreatePlanRequest::create(&mut builder, &CreatePlanRequestArgs{ name: Some(name), plan: Some(plan) }).as_union_value();
        
        let create = Create::create(&mut builder, &CreateArgs { create_type_type: CreateType::CreatePlanRequest, create_type: Some(create) }).as_union_value();
        
        let status = OkStatus::create(&mut builder, &OkStatusArgs { }).as_union_value();
        
        let msg = protocol::Message::create(&mut builder, &MessageArgs{
            data_type: Payload::Create,
            data: Some(create),
            status_type: Status::OkStatus,
            status: Some(status),
        });
        
        builder.finish(msg, None);
        let msg = builder.finished_data();
        
        self.connection.write_all(msg)?;
        
        let res: = self.connection.read()?;
        
        res.
    }
}
