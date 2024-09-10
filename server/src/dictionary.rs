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
            let err_resp = serialize(&DataType::Error("ERR command no recognized".to_owned())).unwrap();
            let response: String = match d_command {
                DataType::Array(o_arr) => {
                    if let Some(arr) = o_arr {
                        if arr.len() == 0 {
                            return err_resp;
                        }
                        if arr[0] == "set" {
                            if arr.len() != 3 {
                                return err_resp;
                            }

                            if let DataType::BulkString(Some(key)) = &arr[1] {
                                if let DataType::BulkString(Some(val)) = &arr[2] {
                                    self.dict.insert(key.clone(), val.clone());
                                    // TODO: check error path here
                                    return f!("{SUCCESS_MSG}");
                                }
                            }
                        }
                        if arr[0] == "get" {
                            if arr.len() != 2 {
                                return err_resp;
                            }

                            if let DataType::BulkString(Some(key)) = &arr[1] {
                                    let val = self.dict.get(key).unwrap();
                                    return serialize(&DataType::BulkString(Some(val.clone()))).unwrap();
                            }
                        }
                    }
        
                    err_resp
                },
                _ => err_resp,
            };
        
            return response;
        }
    }


    
}


