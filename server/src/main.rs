mod dictionary;

use std::sync::{Arc, Mutex};

use dictionary::dictionary::Dictionary;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use utils::deserializer::deserialize;
use utils::serializer::serialize;
use utils::DataType;
use utils::prelude::*;


async fn handle_client(mut socket: TcpStream, redis: &Arc<Mutex<Dictionary>>) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);

    loop {
        let mut buffer = vec![0; 1024];
        match reader.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            },
            Ok(_) => {
                let command = String::from_utf8(buffer).unwrap();
                println!("Received command: {}", command);
                

                let response = {
                    let mut d_command: Option<DataType> = None;
                    match deserialize(&command) {
                        Ok(val) => d_command = Some(val),
                        Err(e) => {
                            println!("Error: {}", e);
                            let response = serialize(&DataType::Error("ERR error parsing data".to_owned())).unwrap();
                            if let Err(e) = writer.write_all(response.as_bytes()).await {
                                println!("Failed to write to client: {}", e);
                                break;
                            }
                        },
                    }
                    
                    let mut dict = redis.lock().unwrap();
                    dict.handle_command(d_command.unwrap())
                };

                if let Err(e) = writer.write_all(response.as_bytes()).await {
                    println!("Failed to write to client: {}", e);
                    break;
                }
            },
            Err(e) => {
                println!("Failed to read from client: {}", e);
                break;
            },
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Redis Lite server listening on 127.0.0.1:6379");
    let redis: Arc<Mutex<Dictionary>> = Arc::new(Mutex::new(Dictionary::new()));

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);
        let redis_clone = Arc::clone(&redis);

        // Spawn a new task to handle the client connection
        tokio::spawn(async move {
            handle_client(socket, &redis_clone).await;
        });
    }
}
