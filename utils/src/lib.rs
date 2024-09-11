use core::fmt;

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

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::SimpleString(val) => write!(f, "\"{}\"", val),
            DataType::Error(err) => write!(f, "{}", err),
            DataType::Integer(num) => write!(f, "(integer) {}", num),
            DataType::BulkString(Some(val)) => write!(f, "\"{}\"", val),
            DataType::BulkString(None) => write!(f, "(nil)"),
            DataType::Array(Some(arr)) => {
                write!(f, "Array: [")?;
                for (i, elem) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            DataType::Array(None) => write!(f, "Array: (nil)"),
        }
    }
}
