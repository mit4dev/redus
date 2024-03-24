use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use chrono::{prelude::*, Duration};

use crate::{
    command::Command,
    resp::constant::{RESP_NULL_BULK_STR, RESP_TERMINATOR},
};

const PONG: &str = "PONG";
const OK: &str = "OK";

#[derive(Debug)]
pub struct Data {
    val: String,

    /// Timestamp before
    expires_at: Option<u64>,
}

pub type Store = Arc<Mutex<HashMap<String, Data>>>;

#[derive(Debug, Clone)]
pub struct Service {
    store: Store,
}

#[allow(unused)]
impl Service {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub fn execute(&self, command: Command) -> String {
        match command {
            Command::Ping => RespResponseFormatter::to_simple_str(PONG.to_string()),
            Command::Echo(arg) => RespResponseFormatter::to_bulk_str(arg),
            Command::Get(key) => {
                let store = self.store.lock().unwrap();

                match store.get(&key) {
                    Some(data) => match data.expires_at {
                        Some(ex) => {
                            let expires =
                                DateTime::from_timestamp_millis(i64::try_from(ex).unwrap())
                                    .unwrap();
                            let now = Utc::now();

                            expires
                                .timestamp_millis()
                                .ge(&now.timestamp_millis())
                                .then(|| RespResponseFormatter::to_bulk_str(data.val.clone()))
                                .unwrap_or(RESP_NULL_BULK_STR.to_string())
                        }
                        None => RespResponseFormatter::to_bulk_str(data.val.to_owned()),
                    },
                    None => RESP_NULL_BULK_STR.to_string(),
                }
            }
            Command::Set((key, val)) => {
                let mut store = self.store.lock().unwrap();

                store.insert(
                    key,
                    Data {
                        val,
                        expires_at: None,
                    },
                );

                RespResponseFormatter::to_simple_str(OK.to_string())
            }
            Command::SetWithExp {
                kv: (key, val),
                ttl,
                exp_type,
                options,
            } => {
                let mut store = self.store.lock().unwrap();

                // TODO: calculate exp type!
                let expires_at =
                    Utc::now() + Duration::try_milliseconds(ttl.parse::<i64>().unwrap()).unwrap();

                store.insert(
                    key,
                    Data {
                        val,
                        expires_at: Some(expires_at.timestamp_millis() as u64),
                    },
                );

                RespResponseFormatter::to_simple_str(OK.to_string())
            }
            Command::Info(arg) => {
                println!("info arg: {:?}", arg);

                RespResponseFormatter::to_bulk_str("role:master".to_string())
            }
        }
    }
}

struct RespResponseFormatter;

impl RespResponseFormatter {
    fn to_bulk_str(value: String) -> String {
        format!(
            "${len}{terminator}{val}{terminator}",
            len = value.len(),
            val = value,
            terminator = RESP_TERMINATOR
        )
        .to_string()
    }

    fn to_simple_str(value: String) -> String {
        format!(
            "+{val}{terminator}",
            val = value,
            terminator = RESP_TERMINATOR
        )
    }
}

#[cfg(test)]
mod tests {
    use super::RespResponseFormatter;

    #[test]
    fn simple_str() {
        let input = "OK";
        let result = RespResponseFormatter::to_simple_str(input.to_string());

        assert_eq!(result, "+OK\r\n");
    }

    #[test]
    fn bulk_str() {
        let input = "blueberry";
        let result = RespResponseFormatter::to_bulk_str(input.to_string());

        assert_eq!(result, "$9\r\nblueberry\r\n");
    }
}
