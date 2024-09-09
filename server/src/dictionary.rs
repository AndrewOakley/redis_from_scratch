use utils::DataType;
use utils::serializer::serialize;
use utils::prelude::*;

pub mod dictionary {
    use std::collections::HashMap;

    use super::*;

    const SUCCESS_MSG: &'static str = "+OK\r\n";

    pub struct Dictionary {
        dict: HashMap<String, String>,
    }

    impl Dictionary {
        pub fn new() -> Self {
            Self {
                dict: HashMap::new(),
            }
        }

        pub fn handle_command(&mut self, d_command: DataType) -> String {
            let response: String = match d_command {
                DataType::Array(o_arr) => {
                    if let Some(arr) = o_arr {
                        if arr[0] == "set" {
                            self.dict.insert("test".to_string(), "out".to_string());
                            println!("Hello andy");
                        }
                        if arr[0] == "get" {
                            let t = self.dict.get("test");
                            println!("Hello andy, {}", t.unwrap());
                        }
                    }
        
                    f!("{SUCCESS_MSG}")
                },
                _ => serialize(&DataType::Error("ERR command no recognized".to_owned())).unwrap(),
            };
        
            return response;
        }
    }


    
}


