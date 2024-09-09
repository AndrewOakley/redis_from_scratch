//! Deserialze RESP protocol values
use crate::prelude::*;
use crate::DataType;


pub fn deserialize(input: &str) -> Result<DataType> {
    match deserialize_helper(input) {
        Ok(result) => Ok(result.0),
        Err(e) => Err(e),
    }
}

fn deserialize_helper(input: &str) -> Result<(DataType, usize)> {
    match input.chars().nth(0) {
        Some(resp_type) => match resp_type {
            '+' => {
                match parse_crlf( &input[1..]) {
                    Ok(val) => Ok((DataType::SimpleString(val.0), val.1+1)),
                    Err(e) => Err(e),
                }
            },
            '-' => {
                match parse_crlf( &input[1..]) {
                    Ok(val) => Ok((DataType::Error(val.0), val.1+1)),
                    Err(e) => Err(e),
                }
            },
            ':' => {
                match input.find("\r\n") {
                    Some(crlf_loc) => {
                        let num_part = &input[1..crlf_loc];
                        match num_part.parse::<i64>() {
                            Ok(val) => Ok((DataType::Integer(val), crlf_loc+2)),
                            Err(_) => Err(Error::ParseError(f!("integer parse"))),
                        }
                    },
                    _ => Err(Error::ParseError(f!("integer parse crlf error"))),
                }
            },
            '$' => {
                match input.find("\r\n") {
                    Some(crlf_loc) => {
                        let len: i64 = input[1..crlf_loc]
                            .parse()
                            .map_err(|_| Error::ParseError(f!("bulk string length parse")))?;


                        if len == -1 {
                            return Ok((DataType::BulkString(None), crlf_loc+2));
                        }

                        if len < 0 {
                            return Err(Error::ParseError(f!("bulk string parse")));
                        }

                        let ulen: usize = usize::try_from(len).unwrap();
                        let start_pos = crlf_loc + 2;
                        let end_pos = start_pos + ulen + 2;

                        if end_pos > input.len() {
                            return Err(Error::ParseError(f!("out of bounds bulk parse")));
                        }

                        match parse_crlf( &input[start_pos..end_pos]) {
                            Ok(val) => Ok((DataType::BulkString(Some(val.0)), end_pos)),
                            Err(e) => Err(e),
                        }
                    },
                    _ => Err(Error::ParseError(f!("bulk string parse")))
                }
            },
            '*' => {
                match input.find("\r\n") {
                    Some(crlf_loc) => {
                        let len: i64 = input[1..crlf_loc]
                            .parse()
                            .map_err(|_| Error::ParseError(f!("array parse invalid length")))?;

                        if len == -1 {
                            return Ok((DataType::Array(None), crlf_loc+2));
                        }

                        if len < 0 {
                            return Err(Error::ParseError(f!("array parse invalid length")));
                        }

                        if len == 0 {
                            return Ok((DataType::Array(Some(Vec::new())), crlf_loc+1));
                        }

                        let ulen: usize = usize::try_from(len).unwrap();
                        let mut cur_len: usize = 0;
                        let mut input_index = crlf_loc+2;
                        let mut result_array: Vec<DataType> = Vec::new();

                        while cur_len < ulen {
                            match deserialize_helper(&input[input_index..]) {
                                Ok(result) => {
                                    result_array.push(result.0);
                                    input_index += result.1;
                                },
                                Err(e) => return Err(e),
                            }

                            cur_len += 1;
                        }

                        Ok((DataType::Array(Some(result_array)), input_index))
                    },
                    _ => Err(Error::ParseError(f!("array parse crlf not found")))
                }
            },
            _ => Err(Error::IdentifierInvalid),
        },
        _ => Err(Error::EmptyInput),
    }
}

fn parse_crlf(input: &str) -> Result<(String, usize)> {
    if input.len() < 2 {
        return Err(Error::ParseError(f!("length error Basic parse")));
    }

    match input.find("\r\n") {
        Some(crlf_loc) => Ok((input[..crlf_loc].to_string(), crlf_loc+2)),
            _ => Err(Error::ParseError(f!("mssing crlf Basic parse"))),
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
    fn arrays_happy() {
        let tests = [
            "*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n",
            "*4\r\n:1\r\n:2\r\n:3\r\n$5\r\nhello\r\n",
            "*1\r\n$4\r\nping\r\n",
            "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n",
            "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n",
            "*-1\r\n",
            "*0\r\n",
        ];
        let expected = [
            Some(vec![DataType::BulkString(Some("hello".to_owned())), DataType::BulkString(Some("world".to_owned()))]),
            Some(vec![DataType::Integer(1), DataType::Integer(2), DataType::Integer(3), DataType::BulkString(Some("hello".to_owned()))]),
            Some(vec![DataType::BulkString(Some("ping".to_owned()))]),
            Some(vec![DataType::BulkString(Some("echo".to_owned())), DataType::BulkString(Some("hello world".to_owned()))]),
            Some(vec![DataType::BulkString(Some("get".to_owned())), DataType::BulkString(Some("key".to_owned()))]),
            None,
            Some(vec![]),
        ];

        for (test, expect)  in zip(tests, expected) {
            let result = deserialize(test).unwrap();
            match result {
                DataType::Array(res) => {
                    for val in zip(res, expect) {
                        assert_eq!(val.0, val.1);
                    }
                },
                _ => panic!(),
            }
        }
    }

    #[test]
    fn nested_arrays_happy() {
        let tests = [
            "*2\r\n*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n*4\r\n:1\r\n:2\r\n:3\r\n$5\r\nhello\r\n",
            ];
        let expected: [Vec<DataType>; 1] = [
            vec![
                DataType::Array(Some(vec![
                    DataType::BulkString(Some("hello".to_owned())),
                    DataType::BulkString(Some("world".to_owned()))]
                )),
                DataType::Array(Some(vec![
                    DataType::Integer(1), DataType::Integer(2),
                    DataType::Integer(3), DataType::BulkString(Some("hello".to_owned()))]
                ))
            ]
        ];

        for (test, expect)  in zip(tests, expected) {
            let result = deserialize(test).unwrap();
            match result {
                DataType::Array(Some(res)) => {
                    for outer in zip(res, expect) {
                        match outer {
                            (DataType::Array(Some(r)), DataType::Array(Some(e))) => {
                                for inner in zip(r, e) {
                                    assert_eq!(inner.0, inner.1);
                                }
                            },
                            _ => panic!(),
                        }
                    }
                },
                _ => panic!(),
            }
        }
    }

    #[test]
    fn deserialize_unhappy() {
        let tests = [
            "+OK\n", "+\r", "", ":\r\n", ":andy\r\n", "$test\r\n",
            "$\r\ntest\r\n", "*2\r\n:10\r\n",
        ];
    
        for test in tests.iter() {
            let result = deserialize(test);
            match result {
                Err(e) => println!("{e}"),
                _ => panic!(),
            }
        }
    }
}
// endregion: --- tests
