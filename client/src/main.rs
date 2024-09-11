use std::io::prelude::*;
use std::net::TcpStream;

use utils::serializer::serialize;
use utils::DataType;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    loop {
        let mut line = String::new();
        print!("127.0.0.1:6379> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();

        let mut input_arr: Vec<DataType> = Vec::new();

        let mut buf = String::new();
        let mut is_quote = false;
        for each_char in line.chars().into_iter() {
            if each_char == ' ' && buf.len() > 0 && !is_quote {
                let serialized = DataType::BulkString(Some(buf.to_owned()));
                input_arr.push(serialized);
                buf.clear();
            } else if each_char == '"' {
                if is_quote {
                    let serialized = DataType::BulkString(Some(buf.to_owned()));
                    input_arr.push(serialized);
                    buf.clear();
                }

                is_quote = !is_quote;
            } else if each_char.is_alphanumeric() || (each_char == ' ' && is_quote) {
                buf.push(each_char);
            }
        }

        if buf.len() > 0 && !is_quote {
            let serialized = DataType::BulkString(Some(buf.to_owned()));
            input_arr.push(serialized);
        }

        if input_arr.len() != 0  {
            let input_serialized = serialize(&DataType::Array(Some(input_arr))).unwrap();
            stream.write(&input_serialized.as_bytes())?;

            let mut buf = vec![0; 1024];
            stream.read(&mut buf).unwrap();

            println!("{}", String::from_utf8(buf).unwrap());
        }
    }
}