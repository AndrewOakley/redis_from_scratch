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

    #[derive(Clone)]
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

        fn get_value(&mut self, key: &String) -> Option<String> {
            let o_val = self.dict.get(key);
            match o_val {
                Some(val) => {
                    if val.is_expire() {
                        self.dict.remove(key);
                        return None;
                    }

                    return Some(val.value.clone());
                },
                _ => return None,
            }
        }

        fn incr_decr_value(&mut self, key: &String, change_type: &DataType) -> Option<i64> {
            let is_incr = if *change_type == "incr" { true } else { false };

            let mut has_error = false;
            let mut new_val = if is_incr { 1 } else { -1 };
            self.dict.entry(key.to_string()).and_modify(|cur_exp| {
                if let Ok(i_val) = cur_exp.value.parse::<i64>() {
                    new_val = if is_incr { i_val + 1 } else { i_val - 1 };
                    cur_exp.value = f!("{new_val}");
                } else {
                    has_error = true;
                }
            }).or_insert(ExpireValue::no_expire(f!("{}", new_val)));

            if !has_error {
                return Some(new_val);
            }

            None
        }

        fn decr_value(&mut self, key: &String) -> Option<i64> {
            let mut has_error = false;
            let mut new_val = -1;
            self.dict.entry(key.to_string()).and_modify(|cur_exp| {
                if let Ok(i_val) = cur_exp.value.parse::<i64>() {
                    new_val = i_val - 1;
                    cur_exp.value = f!("{new_val}");
                } else {
                    has_error = true;
                }
            }).or_insert(ExpireValue::no_expire(f!("-1")));

            if !has_error {
                return Some(new_val);
            }

            None
        }

        fn delete_value(&mut self, key: &String) -> Option<ExpireValue> {
            return self.dict.remove(key);
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
                                    let o_val = self.get_value(key);
                                    match o_val {
                                        Some(val) => return serialize(&DataType::BulkString(Some(val))).unwrap(),
                                        _ => return serialize(&DataType::BulkString(None)).unwrap()
                                    }
                            }
                        }
                        if arr[0] == "exists" || arr[0] == "del" {
                            if arr.len() == 1 {
                                return err_resp;
                            }

                            let mut count = 0;
                            for each_val in &arr[1..] {
                                match each_val {
                                    DataType::BulkString(Some(key)) => {
                                        if arr[0] == "exists" {
                                            if let Some(_) = self.get_value(key) { count += 1; }
                                        }

                                        if arr[0] == "del" {
                                            if let Some(_) = self.delete_value(key) { count += 1; }
                                        }
                                    },
                                    _ => (),
                                }
                            }

                            return serialize(&DataType::Integer(count)).unwrap();
                        }
                        if arr[0] == "incr" || arr[0] == "decr" {
                            if arr.len() != 2 {
                                return err_resp;
                            }

                            if let DataType::BulkString(Some(key)) = &arr[1] {
                                    if let Some(i_val) = self.incr_decr_value(key, &arr[0]) {
                                        return serialize(&DataType::Integer(i_val)).unwrap();
                                    } else {
                                        return serialize(&DataType::Error(f!("ERR value is not an integer or out of range"))).unwrap();
                                    }
                            }

                            return err_resp;
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


