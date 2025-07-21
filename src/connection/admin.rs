use crate::connection::Connection;
use crate::messages::{CreatePlan, DeletePlan, Plan, Plans, StartPlan, StopPlan};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::ops::{Deref, DerefMut};
use track_rails::message_generated::protocol::{ByName, ByNameArgs, CreatePlanRequest, CreatePlanRequestArgs, DeletePlanRequest, DeletePlanRequestArgs, Filter, FilterArgs, FilterType, GetPlansRequest, GetPlansRequestArgs, Message, MessageArgs, OkStatus, OkStatusArgs, Payload, StartPlanRequest, StartPlanRequestArgs, Status, StopPlanRequest, StopPlanRequestArgs};

pub struct Admin{
    connection: Connection,
}


impl Admin {
    pub(crate) fn new(connection: Connection) -> Self {
        Admin{ connection }
    }

    /// Start an already created plan.
    pub fn start_plan(&mut self, id: usize) -> Result<(), String> {
        let mut builder = FlatBufferBuilder::new();

        let start = StartPlanRequest::create(&mut builder, &StartPlanRequestArgs{ id: id as u64 });

        let msg = Self::wrap_msg(&mut builder, Payload::StartPlanRequest, start.as_union_value());

        self.write_all(&msg)?;

        let _: StartPlan = self.receive()?;
        Ok(())
    }

    /// Try to stop a created plan.
    pub fn stop_plan(&mut self, id: usize) -> Result<(), String> {
        let mut builder = FlatBufferBuilder::new();

        let stop = StopPlanRequest::create(&mut builder, &StopPlanRequestArgs{ id: id as u64 });

        let msg = Self::wrap_msg(&mut builder, Payload::StopPlanRequest, stop.as_union_value());

        self.write_all(&msg)?;

        let _: StopPlan = self.receive()?;
        Ok(())
    }

    fn wrap_msg(builder: &mut FlatBufferBuilder, payload: Payload,data: WIPOffset<flatbuffers::UnionWIPOffset>) -> Vec<u8> {
        let status = OkStatus::create(builder, &OkStatusArgs{});

        let msg = Message::create(builder, &MessageArgs {
            data_type: payload,
            data: Some(data),
            status_type: Status::OkStatus,
            status: Some(status.as_union_value()),
        });
        builder.finish(msg, None);
        builder.finished_data().to_vec()
    }


    /// Create a new plan with a given name and according to the provided schema.
    ///
    /// # Arguments
    ///
    /// * `name`: the name of the plan
    /// * `plan`: the schema of the plan
    ///
    /// returns: Result<usize, String>
    pub fn create_plan<Name: AsRef<str>, Plan: AsRef<str>>(&mut self, name: Name, plan: Plan) -> Result<usize, String> {
        let mut builder = FlatBufferBuilder::new();

        let name = builder.create_string(name.as_ref());
        let plan = builder.create_string(plan.as_ref());

        let create = CreatePlanRequest::create(&mut builder, &CreatePlanRequestArgs{ name: Some(name), plan: Some(plan) });

        let msg = Self::wrap_msg(&mut builder, Payload::CreatePlanRequest, create.as_union_value());

        self.write_all(&msg)?;

        let res: CreatePlan = self.receive()?;

        Ok(res.id)
    }

    /// Get all registered plans, running as well as stopped.
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

        let msg = Self::wrap_msg(&mut builder, Payload::GetPlansRequest, get_plan);

        self.write_all(&msg)?;

        // wait response
        let res: Plans = self.receive()?;

        Ok(res.0)

    }

    /// Delete a specified plan.
    pub fn delete_plan(&mut self, id: usize) -> Result<(), String> {
        let mut builder = FlatBufferBuilder::new();

        // send request
        let delete = DeletePlanRequest::create(&mut builder, &DeletePlanRequestArgs{ id: id as u64, });

        let msg = Self::wrap_msg(&mut builder, Payload::DeletePlanRequest, delete.as_union_value());

        self.write_all(&msg)?;

        // wait for response
        let _: DeletePlan = self.receive()?;

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
