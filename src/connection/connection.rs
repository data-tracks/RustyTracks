use crate::connection::admin::Admin;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use flatbuffers::FlatBufferBuilder;
use tracing::{debug, error, info};
use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::{Disconnect, DisconnectArgs, MessageArgs, OkStatus, OkStatusArgs, Payload, RegisterRequest, RegisterRequestArgs, Status, Text, TextArgs, Time, TimeArgs, Train, TrainArgs, Value as ProtoValue, ValueWrapper, ValueWrapperArgs};
use crate::connection::Permission::AdminPermission;
use crate::connection::permission::Permission;
use crate::messages;
use crate::messages::Message;
use crate::value::Value;

pub struct Connection {
    id: Option<usize>,
    host: String,
    port: u16,
    stream: TcpStream,
    permissions: Vec<Permission>
}

impl Connection {

    pub(crate) fn new(host: &str, port: u16, stream: TcpStream) -> Result<Self, String> {
        let host = String::from(host);

        let mut connection = Connection{
            id: None,
            host,
            port,
            stream,
            permissions: vec![],
        };
        connection.connect()?;
        Ok(connection)
    }


    pub fn send<V:Into<Value>>(&mut self, msg: V) -> Result<(), String> {
        let msg = self.wrap_send(msg.into());
        self.write_all(&msg)
    }

    pub fn receive(&mut self) -> Result<Message, String> {
        self.read_msg()
    }

    pub fn admin(self) -> Result<Admin, String> {
        if !self.permissions.contains(&AdminPermission) {
            return Err(String::from("No admin permission"));
        }
        Ok(Admin::new(self))
    }

    pub(crate) fn write_all<'a>(&'a mut self, msg: &'a [u8]) -> Result<(), String> {
        let length: [u8; 4] = (msg.len() as u32).to_be_bytes();
        // we write length first
        self.stream.write_all(&length).map_err(|e| e.to_string())?;
        debug!("sending {} bytes", msg.len());
        // then msg
        self.stream.write_all(msg).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn connect(&mut self) -> Result<(), String> {
        let mut builder = FlatBufferBuilder::new();
        let register = RegisterRequest::create(&mut builder, &RegisterRequestArgs { id: None, catalog: None }).as_union_value();

        let status = OkStatus::create(&mut builder, &OkStatusArgs{}).as_union_value();

        let msg = protocol::Message::create(&mut builder, &MessageArgs{ data_type: Payload::RegisterRequest, data: Some(register), status_type: Status::OkStatus, status: Some(status) });

        builder.finish(msg, None);
        let msg = builder.finished_data().to_vec();


        let code = self.write_all(&msg).map_err(|e| e.to_string());
        match code {
            Ok(_) => info!("Connected successfully"),
            _ => error!("Error writing to stream"),
        }


        let msg: messages::RegisterResponse = self.read_msg()?;
        debug!("{:?}", msg);
        self.permissions = msg.permissions;
        Ok(())
    }

    pub fn read_msg<Msg>(&mut self) -> Result<Msg, String> where
        Msg: for<'a> TryFrom<protocol::Message<'a>, Error = String> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf).map_err(|e| e.to_string())?;


        let length = u32::from_be_bytes(buf) as usize;
        let mut buffer = vec![0u8; length];
        self.stream.read_exact(&mut buffer).map_err(|err| err.to_string())?;
        let msg = flatbuffers::root::<protocol::Message>(&buffer).map_err(|e| e.to_string())?;
        Msg::try_from(msg)
    }

    pub(crate) fn wrap_send(&mut self, msg: Value) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();

        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let time = Time::create(&mut builder, &TimeArgs{data: millis as u64 as i64 });

        let topic = builder.create_string("");

        let value = msg.flatternize(&mut builder);

        let values = builder.create_vector(&[value]);

        let train = Train::create(&mut builder, &TrainArgs {
            values: Some(values),
            topic: Some(topic),
            event_time: Some(time),
        }).as_union_value();

        let status = OkStatus::create(&mut builder, &OkStatusArgs{}).as_union_value();

        let msg = protocol::Message::create(&mut builder, &MessageArgs{ data_type: Payload::Train, data: Some(train), status_type: Status::OkStatus, status: Some(status) }).as_union_value();

        builder.finish(msg, None);
        builder.finished_data().to_vec()
    }

}

impl Drop for Connection {
    fn drop(&mut self) {
        let mut builder = FlatBufferBuilder::new();

        let diconnect = Disconnect::create(&mut builder, &DisconnectArgs { id: self.id.unwrap_or_default() as u64 }).as_union_value();

        let status = OkStatus::create(&mut builder, &OkStatusArgs{}).as_union_value();

        let message = protocol::Message::create(&mut builder, &MessageArgs{data_type: Payload::Disconnect, data: Some(diconnect), status_type: Status::OkStatus, status: Some(status) }).as_union_value();

        builder.finish(message, None);
        let msg = builder.finished_data().to_vec();
        debug!("disconnecting");
        self.write_all(&msg).unwrap()
    }
}