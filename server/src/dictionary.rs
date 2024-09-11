use utils::DataType;
use utils::serializer::serialize;
use utils::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod dictionary {
    use std::collections::HashMap;

    use super::*;

    const SUCCESS_MSG: &'static str = "+OK\r\n";

    pub struct Dictionary {
        dict: HashMap<String, ExpireValue>,
    }

    struct ExpireValue {
        value: String,
        exp: Option<u128>,
    }

    impl ExpireValue {
        fn no_expire(value: String) -> Self {
            Self {
                value,
                exp: None,
            }
        }

        fn expire_seconds(value: String, exp: u128) -> Self {
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            let unix_timestamp = since_the_epoch.as_millis();

            Self {
                value,
                exp: Some(unix_timestamp + (exp * 1000)),
            }
        }

        fn expire_millis(value: String, exp: u128) -> Self {
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
            let unix_timestamp = since_the_epoch.as_millis();

            Self {
                value,
                exp: Some(unix_timestamp + exp),
            }
        }

        fn specific_expire_seconds(value: String, exp: u128) -> Self {
            Self {
                value,
                exp: Some(exp * 1000),
            }
        }

        fn specific_expire_millis(value: String, exp: u128) -> Self {
            Self {
                value,
                exp: Some(exp),
            }
        }

        fn is_expire(&self) -> bool {
            if let Some(exp) = self.exp  {
                let start = SystemTime::now();
                let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
                let unix_timestamp = since_the_epoch.as_millis();

                return unix_timestamp > exp;
            }

            return false;
        }
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
                            if arr.len() == 3 {
                                if let DataType::BulkString(Some(key)) = &arr[1] {
                                    if let DataType::BulkString(Some(val)) = &arr[2] {
                                        self.dict.insert(key.clone(), ExpireValue::no_expire(val.clone()));
                                        return f!("{SUCCESS_MSG}");
                                    }
                                }
                            }

                            if arr.len() == 5 {
                                if let DataType::BulkString(Some(key)) = &arr[1] {
                                    if let DataType::BulkString(Some(val)) = &arr[2] {
                                        if let DataType::BulkString(Some(exp_com)) = &arr[3] {
                                            if let DataType::BulkString(Some(exp_time)) = &arr[4] {
                                                let exp_time: u128 = exp_time.parse().unwrap();
                                                match exp_com.as_str() {
                                                    "EX" => {
                                                        self.dict.insert(key.clone(), ExpireValue::expire_seconds(val.clone(), exp_time));
                                                        return f!("{SUCCESS_MSG}");
                                                    },
                                                    "PX" => {
                                                        self.dict.insert(key.clone(), ExpireValue::expire_millis(val.clone(), exp_time));
                                                        return f!("{SUCCESS_MSG}");
                                                    },
                                                    "EXAT" => {
                                                        self.dict.insert(key.clone(), ExpireValue::specific_expire_seconds(val.clone(), exp_time));
                                                        return f!("{SUCCESS_MSG}");
                                                    },
                                                    "PXAT" => {
                                                        self.dict.insert(key.clone(), ExpireValue::specific_expire_millis(val.clone(), exp_time));
                                                        return f!("{SUCCESS_MSG}");
                                                    },
                                                    _ => (),
                                                }
                                            }
                                        }
                                    }
                                }
                            }


                            return err_resp;
                        }
                        if arr[0] == "get" {
                            if arr.len() != 2 {
                                return err_resp;
                            }

                            if let DataType::BulkString(Some(key)) = &arr[1] {
                                    let o_val = self.dict.get(key);
                                    match o_val {
                                        Some(val) => {
                                            if val.is_expire() {
                                                self.dict.remove(key);
                                                return serialize(&DataType::BulkString(None)).unwrap()
                                            }

                                            return serialize(&DataType::BulkString(Some(val.value.clone()))).unwrap()
                                        },
                                        _ => return serialize(&DataType::BulkString(None)).unwrap()
                                    }
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


