use std::fmt;

pub enum DataTypes {
    SimpleString,
    Errors,
    Integers,
    BulkStrings,
    Arrays,
}

impl fmt::Display for DataTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_name = match self {
            DataTypes::SimpleString => "SimpleString",
            DataTypes::Errors => "Errors",
            DataTypes::Integers => "Integers",
            DataTypes::BulkStrings => "BulkStrings",
            DataTypes::Arrays => "Arrays",
        };
        write!(f, "{}", variant_name)
    }
}

pub struct DeserializedValue {
    pub val: String,
    pub val_type: DataTypes,
}

pub fn deserialize(input: &str) -> Result<DeserializedValue, String> {
    let data_type_char = input.chars().nth(0);

    let mut result: String = String::new();
    match data_type_char {
        Some(byte) => match byte {
            '+' => parse_simple_string(&mut result, &input[1..]),
            _ => return Err(String::from("Invalid first character")),
        }
        None => return Err(String::from("Empty string found")),
    };


    Ok(DeserializedValue {
        val: result,
        val_type: DataTypes::SimpleString,
    })
}

fn parse_simple_string(result: &mut String, input: &str) {
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
     *result = buf;
    }
}