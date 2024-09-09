mod deserializer;
mod serializer;

pub use deserializer::*;
pub use serializer::*;

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Option<Vec<DataType>>),
}
