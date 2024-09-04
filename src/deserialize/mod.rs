use std::fmt;


pub enum DataType {
    SimpleString,
    Errors,
    Integers,
    BulkStrings,
    Arrays,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_name = match self {
            DataType::SimpleString => "SimpleString",
            DataType::Errors => "Errors",
            DataType::Integers => "Integers",
            DataType::BulkStrings => "BulkStrings",
            DataType::Arrays => "Arrays",
        };
        write!(f, "{}", variant_name)
    }
}

pub struct DeserializedValue {
    val: Box<String>,
    data_type: DataType,
}

pub fn deserialize(input: &str) -> Result<DeserializedValue, String> {
    let deserialzed = match input.chars().nth(0) {
        Some(byte) => match byte {
            '+' => parse_simple_string( &input[1..]),
            _ => Err(String::from("First character invalid")),
        }
        None => Err(String::from("Empty input")),
    };


    deserialzed
}

fn parse_simple_string(input: &str) -> Result<DeserializedValue, String> {
    let mut buf = String::new();
    let mut has_error = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                match chars.peek() {
                    Some('\n') => break,
                    _ => {
                        has_error = true;
                        break;
                    },
                }
            }
            '\n' => {
                has_error = true;
                break;
            },
            _ => buf.push(c),
        }
    }

    if !has_error {
     return Ok(DeserializedValue {
        val: Box::new(buf),
        data_type: DataType::SimpleString,
     });
    }

    return Err(format!("Could not parse data for {}", input))
}


#[cfg(test)]
mod tests {
    use crate::*
}