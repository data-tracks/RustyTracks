use std::io::{Read, Write};
use schemas::message_generated::protocol::{String as Text, Message, MessageArgs, Payload, Register, RegisterArgs, StringBuilder, Time, TimeArgs, Train, TrainArgs, ValueWrapper, ValueWrapperArgs, StringArgs, Value, ValueUnionTableOffset, ValueWrapperBuilder};
use std::net::TcpStream;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use flatbuffers::FlatBufferBuilder;
use tracing::{error, info};

struct Connection {
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

    pub(crate) fn send(&mut self, msg: &str) {

    }

    fn connect(&mut self) -> Result<(), String> {
        let mut builder = FlatBufferBuilder::new();
        let register = Register::create(&mut builder, &RegisterArgs { id: None, catalog: None, ..Default::default() }).as_union_value();
        let msg = Message::create(&mut builder, &MessageArgs{ data_type: Payload::Register, data: Some(register), status: None }).as_union_value();

        builder.finish(msg, None);
        let msg = builder.finished_data().to_vec();

        let code = self.stream.write(&msg).map_err(|e| e.to_string())?;
        match code {
            200 => info!("Connected successfully"),
            _ => error!("Error writing to stream {}", code),
        }
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf).map_err(|e| e.to_string())?;


        let length = u32::from_be_bytes(buf) as usize;
        let mut buffer = vec![0u8; length];
        self.stream.read_exact(&mut buffer).map_err(|err| err.to_string())?;
        let msg = flatbuffers::root::<Message>(&buffer).map_err(|e| e.to_string())?;
        println!("{:?}", msg);
        Ok(())
    }

    fn msg(&mut self, msg: &str) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();

        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let time = Time::create(&mut builder, &TimeArgs{data: millis as u64 });

        let topic = builder.create_string("");

        let value = builder.create_string(msg );
        let value = Text::create(&mut builder, &StringArgs{ data: Some(value) }).as_union_value();


        let value = ValueWrapper::create(&mut builder, &ValueWrapperArgs{ data_type: Value::String, data: Some(value)});

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


struct Client{
    host: String,
    port: u16,
}


impl Client {
    fn new(host: &str, port: u16) -> Self {
        Client{ host: host.to_string(), port }
    }

    fn connect(&self) -> Result<Connection, String> {
        let stream = TcpStream::connect((self.host.clone(), self.port)).unwrap();
        Connection::new(&self.host, self.port, stream)
    }
}

#[cfg(test)]
mod tests{
    use crate::Client;

    #[test]
    fn test_connect(){
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
    }

    #[test]
    fn test_send_values(){
        let client = Client::new("localhost", 9999);
        let mut connection = client.connect().unwrap();
        connection.send("Hello world");
    }
}

