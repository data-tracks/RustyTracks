use flatbuffers::{FlatBufferBuilder, WIPOffset};
use track_rails::message_generated::protocol;
use track_rails::message_generated::protocol::{Bool, BoolArgs, Float, FloatArgs, Integer, IntegerArgs, Null, NullArgs, Text, TextArgs, ValueWrapper, ValueWrapperArgs};

pub enum Value {
    Text(String),
    Number(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl Value{
    pub fn flatternize<'b>(&self, builder: & mut FlatBufferBuilder<'b>) -> WIPOffset<ValueWrapper<'b>> {
        match self {
            Value::Text(t) => {
                let text = builder.create_string(t.as_str());
                let text = Text::create(builder, &TextArgs{data: Some(text)}).as_union_value();
                ValueWrapper::create(builder, &ValueWrapperArgs{ data_type: protocol::Value::Text, data: Some(text) })
            }
            Value::Number(n) => {
                let number = Integer::create(builder, &IntegerArgs{data: *n }).as_union_value();
                ValueWrapper::create(builder, &ValueWrapperArgs{ data_type: protocol::Value::Integer, data: Some(number) })
            }
            Value::Float(f) => {
                let float = Float::create(builder, &FloatArgs{data: *f as f32 }).as_union_value();
                ValueWrapper::create(builder, &ValueWrapperArgs{ data_type: protocol::Value::Float, data: Some(float) })
            }
            Value::Bool(b) => {
                let bool = Bool::create(builder, &BoolArgs{data: *b}).as_union_value();
                ValueWrapper::create(builder, &ValueWrapperArgs{ data_type: protocol::Value::Bool, data: Some(bool) })
            }
            Value::Null => {
                let null = Null::create(builder, &NullArgs{}).as_union_value();
                ValueWrapper::create(builder, &ValueWrapperArgs{ data_type: protocol::Value::Null, data: Some(null) })
            }
        }
        
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value)
    }
}

impl From<f64> for Value {  
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Text(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Text(value.to_string())
    }
}
