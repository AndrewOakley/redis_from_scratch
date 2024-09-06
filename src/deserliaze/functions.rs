use std::fmt;
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub enum DataType {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Vec<DataType>),
}

fn parse_crlf(input: &str) -> Result<String> {
    if input.len() < 2 {
        return Err(Error::ParseError(f!("length error Basic parse, {}", input)));
    }

    let crlf = input.find("\r\n");

    match input.find("\r\n") {
        Some(loc) => Ok(input[..loc].to_string()),
            _ => Err(Error::ParseError(f!("mssing crlf Basic parse, {}", input))),
    }
}

pub fn deserialize(input: &str) -> Result<DataType> {
    match input.chars().nth(0) {
        Some(resp_type) => match resp_type {
            '+' => {
                match parse_crlf( &input[1..]) {
                    Ok(val) => Ok(DataType::SimpleString(val)),
                    Err(e) => Err(e),
                }
            },
            '-' => {
                match parse_crlf( &input[1..]) {
                    Ok(val) => Ok(DataType::Error(val.to_string())),
                    Err(e) => Err(e),
                }
            },
            ':' => {
                let num_part = &input[1..input.len() - 2];
                match num_part.parse::<i64>() {
                    Ok(val) => Ok(DataType::Integer(val)),
                    Err(_) => Err(Error::ParseError(f!("integer parse, {}", input))),
                }
            },
            '$' => {
                let crlf_loc = input.find("\r\n");
                match crlf_loc {
                    Some(loc) => {
                        let len: i64 = input[1..loc]
                            .parse()
                            .map_err(|_| Error::ParseError(f!("bulk string parse, {}", input)))?;

                        if len == -1 {
                            return Ok(DataType::BulkString(None));
                        }

                        if loc + 2 >= input.len() {
                            return Err(Error::ParseError(f!("out of bounds integer parse, {}", input)));
                        }

                        match parse_crlf( &input[loc+2..]) {
                            Ok(val) => Ok(DataType::BulkString(Some(val))),
                            Err(e) => Err(e),
                        }
                    },
                    _ => Err(Error::ParseError(f!("bulk string parse, {}", input)))
                }
            },
            // '*' => {
            //     let crlf_loc = input.find("\r\n");
            //     match crlf_loc {
            //         Some(loc) => {
            //             let len: i64 = input[1..loc]
            //                 .parse()
            //                 .map_err(|_| Error::ParseError(f!("array parse")))?;

            //             if len < 0 {
            //                 return Err(Error::ParseError(f!("array parse")));
            //             }

            //             if len == 0 {
            //                 return Ok(DataType::Array(Vec::new()));
            //             }

            //             if loc + 2 < input.len() {
                            
            //             }
            //         },
            //         _ => Err(Error::ParseError(f!("bulk string parse, {}", input)))
            //     }
            // },
            _ => Err(Error::IdentifierInvalid),
        },
        _ => Err(Error::EmptyInput),
    }
}


// region: --- tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    #[test]
    fn simple_string_happy() {
        let tests = ["+OK\r\n", "+\r\n", "+Andy Oakley\r\n"];
        let expected = ["OK", "", "Andy Oakley"];

        for (test, expect)  in zip(tests, expected) {
            let result = deserialize(test).unwrap();
            assert_eq!(result, DataType::SimpleString(expect.to_string()));
        }
    }

    #[test]
    fn errors_happy() {
        let tests = ["-ERROR message\r\n", "-\r\n", "-Andy Oakley\r\n"];
        let expected = ["ERROR message", "", "Andy Oakley"];

        for (test, expect)  in zip(tests, expected) {
            let result = deserialize(test).unwrap();
            assert_eq!(result, DataType::Error(expect.to_string()));
        }
    }

    #[test]
    fn integers_happy() {
        let tests = [":100\r\n", ":0\r\n", ":-69\r\n", ":+69\r\n"];
        let expected = [100, 0, -69, 69];

        for (test, expect)  in zip(tests, expected) {
            let result = deserialize(test).unwrap();
            assert_eq!(result, DataType::Integer(expect));
        }
    }

    #[test]
    fn bulk_string_happy() {
        let tests = ["$5\r\nhello\r\n", "$0\r\n\r\n", "$-1\r\n"];
        let expected = [Some("hello".to_owned()), Some("".to_owned()), None];

        for (test, expect)  in zip(tests, expected) {
            let result = deserialize(test).unwrap();
            assert_eq!(result, DataType::BulkString(expect));
        }
    }

    #[test]
    fn deserialize_unhappy() {
        let tests = ["+OK\n", "+\r", "", ":\r\n", ":andy\r\n", "$test\r\n", "$\r\ntest\r\n"];

        for test  in tests {
            let result = deserialize(test);
            result.expect_err("Test");
        }
    }
}
// endregion: --- tests
