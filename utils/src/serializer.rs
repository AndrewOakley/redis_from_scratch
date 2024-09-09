//! Serialize RESP protocol values
use crate::prelude::*;
use crate::DataType;


pub fn serialize(input: &DataType) -> Result<String> {
    match input {
        DataType::SimpleString(val) => Ok(f!("+{val}\r\n")),
        DataType::Error(val) => Ok(f!("-{val}\r\n")),
        DataType::Integer(val) => Ok(f!(":{val}\r\n")),
        DataType::BulkString(o_val) => {
            if let Some(val) = o_val {
                return Ok(f!("${}\r\n{}\r\n", val.len(), val));
            }

            return Ok("$-1\r\n".to_string());
        },
        DataType::Array(o_arr) => {
            if let Some(arr) = o_arr {
                let mut serialized = f!("*{}\r\n", arr.len());
                for data in arr {
                    match serialize(data) {
                        Ok(val) => serialized.push_str(&val),
                        Err(e) => return Err(e),
                    }
                }

                return Ok(serialized);
            }

            return Ok("*-1\r\n".to_string());
        }
    }
}


// region: --- tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    #[test]
    fn simple_string_happy() {
        let tests = [DataType::SimpleString(f!("")), DataType::SimpleString(f!("test"))];
        let expected = ["+\r\n", "+test\r\n"];

        for (test, expect)  in zip(tests, expected) {
            let result = serialize(&test).unwrap();
            assert_eq!(&result, expect);
        }
    }

    #[test]
    fn errors_happy() {
        let tests = [DataType::Error(f!("")), DataType::Error(f!("ERROR error"))];
        let expected = ["-\r\n", "-ERROR error\r\n"];

        for (test, expect)  in zip(tests, expected) {
            let result = serialize(&test).unwrap();
            assert_eq!(&result, expect);
        }
    }

    #[test]
    fn integers_happy() {
        let tests = [DataType::Integer(1), DataType::Integer(0), DataType::Integer(-20)];
        let expected = [":1\r\n", ":0\r\n", ":-20\r\n"];

        for (test, expect)  in zip(tests, expected) {
            let result = serialize(&test).unwrap();
            assert_eq!(&result, expect);
        }
    }

    #[test]
    fn bulk_string_happy() {
        let tests = [DataType::BulkString(Some(f!(""))), DataType::BulkString(Some(f!("test"))), DataType::BulkString(None)];
        let expected = ["$0\r\n\r\n", "$4\r\ntest\r\n", "$-1\r\n"];

        for (test, expect)  in zip(tests, expected) {
            let result = serialize(&test).unwrap();
            assert_eq!(&result, expect);
        }
    }

    #[test]
    fn arrays_happy() {
        let tests = [
            DataType::Array(Some(vec![DataType::BulkString(Some("hello".to_owned())), DataType::BulkString(Some("world".to_owned()))])),
            DataType::Array(Some(vec![DataType::Integer(1), DataType::Integer(2), DataType::Integer(3), DataType::BulkString(Some("hello".to_owned()))])),
            DataType::Array(Some(vec![DataType::BulkString(Some("ping".to_owned()))])),
            DataType::Array(Some(vec![DataType::BulkString(Some("echo".to_owned())), DataType::BulkString(Some("hello world".to_owned()))])),
            DataType::Array(Some(vec![DataType::BulkString(Some("get".to_owned())), DataType::BulkString(Some("key".to_owned()))])),
            DataType::Array(None),
            DataType::Array(Some(vec![])),
        ];
        let expected = [
            "*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n",
            "*4\r\n:1\r\n:2\r\n:3\r\n$5\r\nhello\r\n",
            "*1\r\n$4\r\nping\r\n",
            "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n",
            "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n",
            "*-1\r\n",
            "*0\r\n",
        ];

        for (test, expect)  in zip(tests, expected) {
            let result = serialize(&test).unwrap();
            assert_eq!(&result, expect);
        }
    }
}
// endregion: --- tests
