#![allow(unused)] // remove eventually, this is for dev


use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;
mod deserliaze;

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

fn main() -> Result<()> {
    Ok(())
}
