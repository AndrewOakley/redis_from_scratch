mod deserialize;
use crate::deserialize::deserialize;

// For Simple Strings, the first byte of the reply is "+"
// For Errors, the first byte of the reply is "-"
// For Integers, the first byte of the reply is ":"
// For Bulk Strings, the first byte of the reply is "$"
// For Arrays, the first byte of the reply is "*"

// "$-1\r\n"
// "*1\r\n$4\r\nping\r\n”
// "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n”
// "*2\r\n$3\r\nget\r\n$3\r\nkey\r\n”
// "+OK\r\n"
// "-Error message\r\n"
// "$0\r\n\r\n"
// "+hello world\r\n”

fn main() {
    let test_1 = "+OK\r\n".to_string();

    let result_1 = deserialize(&test_1).unwrap();

    println!("Value: {}", result_1.val);
    println!("length: {}", result_1.val.len());
    println!("type: {}", result_1.val_type);
}
