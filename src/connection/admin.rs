use crate::connection::Connection;
use crate::messages::{CreatePlan, DeletePlan, Plan, Plans};
use flatbuffers::FlatBufferBuilder;
use std::ops::{Deref, DerefMut};
use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::{ByName, ByNameArgs, CreatePlanRequest, CreatePlanRequestArgs, DeletePlanRequest, DeletePlanRequestArgs, Filter, FilterArgs, FilterType, GetPlansRequest, GetPlansRequestArgs, Message, MessageArgs, OkStatus, OkStatusArgs, Payload, Status};

pub struct Admin{
    connection: Connection,
}

impl Admin {
    pub(crate) fn new(connection: Connection) -> Self {
        Admin{ connection }
    }

    pub fn create_plan<Name: AsRef<str>, Plan: AsRef<str>>(&mut self, name: Name, plan: Plan) -> Result<usize, String> {
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

        self.write_all(msg)?;

        let res: CreatePlan = self.read()?;

        Ok(res.id)
    }
    
    pub fn get_plans(&mut self) -> Result<Vec<Plan>, String> {
        self.get_plans_by_name("*")
    }

    pub fn get_plans_by_name<S:AsRef<str>>(&mut self, filter: S) -> Result<Vec<Plan>, String> {
        let mut builder = FlatBufferBuilder::new();
        
        let name = builder.create_string(filter.as_ref());
        
        let filter = ByName::create(&mut builder, &ByNameArgs{ name: Some(name) }).as_union_value();
        
        let filter = Filter::create(&mut builder, &FilterArgs{ filter_type_type: FilterType::ByName, filter_type: Some(filter) });
        
        // send request 
        let get_plan = GetPlansRequest::create(&mut builder, &GetPlansRequestArgs { name: Some(filter) }).as_union_value();
        
        let status = OkStatus::create(&mut builder, &OkStatusArgs { }).as_union_value();
        
        let msg = Message::create(&mut builder, &MessageArgs{
            data_type: Payload::GetPlansRequest,
            data: Some(get_plan),
            status_type: Status::OkStatus,
            status: Some(status),
        });
        
        builder.finish(msg, None);
        let msg = builder.finished_data().to_vec();
        self.write_all(&msg)?;
        
        // wait response
        let res: Plans = self.read()?;
        
        Ok(res.0)
        
    }

    pub fn delete_plan(&mut self, id: usize) -> Result<(), String> {
        let mut builder = FlatBufferBuilder::new();
        
        // send request
        let delete = DeletePlanRequest::create(&mut builder, &DeletePlanRequestArgs{ id: id as u64, }).as_union_value();
        let status = OkStatus::create(&mut builder, &OkStatusArgs { }).as_union_value();
        let msg = Message::create(&mut builder, &MessageArgs{
            data_type: Payload::DeletePlanRequest,
            data: Some(delete),
            status_type: Status::OkStatus,
            status: Some(status),
        });
        builder.finish(msg, None);
        let msg = builder.finished_data().to_vec();
        self.write_all(&msg)?;
        
        // wait for response
        let _: DeletePlan = self.read()?;
        
        Ok(())
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
