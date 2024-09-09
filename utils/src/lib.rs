pub mod deserializer;
pub mod direntry_froms;
pub mod serializer;
pub mod prelude;
pub mod error;

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Option<Vec<DataType>>),
}

impl PartialEq<&str> for DataType {
    fn eq(&self, other: &&str) -> bool {
        match self {
            DataType::SimpleString(s) => s == *other,
            DataType::Error(e) => e == *other,
            DataType::BulkString(Some(s)) => s == *other,
            DataType::BulkString(None) => *other == "",
            _ => false, // For Integer or Array, return false
        }
    }
}