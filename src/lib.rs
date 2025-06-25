mod test;

use flatbuffers::FlatBufferBuilder;
use schemas::message_generated::protocol::{Message, MessageArgs, Payload, RegisterRequest, RegisterRequestArgs, Status, StatusArgs, Text, TextArgs, Time, TimeArgs, Train, TrainArgs, Value, ValueWrapper, ValueWrapperArgs};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info};
use tracing::field::debug;

pub struct Connection {
    host: String,
    port: u16,
    stream: TcpStream,
}


impl Connection {

    fn new(host: &str, port: u16, stream: TcpStream) -> Result<Self, String> {
        let host = String::from(host);

        let mut connection = Connection{
            host,
            port,
            stream,
        };
        connection.connect()?;
        Ok(connection)
    }

    pub(crate) fn send(&mut self, msg: &str) -> Result<(), String> {
        let msg = self.msg(msg);
        self.write_all(&msg)
    }

    pub(crate) fn receive(&mut self) -> Result<String, String> {
        self.read()
    }

    fn write_all<'a>(&'a mut self, msg: &'a [u8]) -> Result<(), String> {
        let length: [u8; 4] = (msg.len() as u32).to_be_bytes();
        // we write length first
        self.stream.write_all(&length).map_err(|e| e.to_string())?;
        println!("sending {} bytes", msg.len());
        // then msg
        self.stream.write_all(msg).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn connect(&mut self) -> Result<(), String> {
        let mut builder = FlatBufferBuilder::new();
        let register = RegisterRequest::create(&mut builder, &RegisterRequestArgs { id: None, catalog: None }).as_union_value();

        let status = builder.create_string("");
        let status = Status::create(&mut builder, &StatusArgs{ msg: Some(status) });
        
        let msg = Message::create(&mut builder, &MessageArgs{ data_type: Payload::RegisterRequest, data: Some(register), status: Some(status) });

        builder.finish(msg, None);
        let msg = builder.finished_data().to_vec();


        let code = self.write_all(&msg).map_err(|e| e.to_string())?;
        match code {
            () => info!("Connected successfully"),
            _ => error!("Error writing to stream"),
        }


        let msg = self.read()?;
        println!("{:?}", msg);
        Ok(())
    }

    fn read(&mut self) -> Result<String, String> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf).map_err(|e| e.to_string())?;


        let length = u32::from_be_bytes(buf) as usize;
        let mut buffer = vec![0u8; length];
        self.stream.read_exact(&mut buffer).map_err(|err| err.to_string())?;
        let msg = flatbuffers::root::<Message>(&buffer).map_err(|e| e.to_string())?;
        Ok(format!("{:?}", msg))
    }

    fn msg(&mut self, msg: &str) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();

        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let time = Time::create(&mut builder, &TimeArgs{data: millis as u64 as i64 });

        let topic = builder.create_string("");

        let value = builder.create_string(msg );
        let value = Text::create(&mut builder, &TextArgs{ data: Some(value) }).as_union_value();


        let value = ValueWrapper::create(&mut builder, &ValueWrapperArgs{ data_type: Value::Text, data: Some(value)});

        let values = builder.create_vector(&[value]);

        let train = Train::create(&mut builder, &TrainArgs {
            values: Some(values),
            topic: Some(topic),
            event_time: Some(time),
        }).as_union_value();
        let msg = Message::create(&mut builder, &MessageArgs{ data_type: Payload::Train, data: Some(train), status: None }).as_union_value();

        builder.finish(msg, None);
        builder.finished_data().to_vec()
    }


}


pub struct Client{
    host: String,
    port: u16,
}


impl Client {
    pub fn new(host: &str, port: u16) -> Self {
        Client{ host: host.to_string(), port }
    }

    pub fn connect(&self) -> Result<Connection, String> {
        let stream = TcpStream::connect((self.host.clone(), self.port)).unwrap();
        Connection::new(&self.host, self.port, stream)
    }
}

