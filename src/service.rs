use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    command::Command,
    resp::constant::{RESP_NULL_BULK_STR, RESP_TERMINATOR},
};

const PONG: &str = "PONG";
const OK: &str = "OK";

#[derive(Debug, Clone)]
pub struct Service {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl Service {
    pub fn new(store: Arc<Mutex<HashMap<String, String>>>) -> Self {
        Self { store }
    }

    pub fn execute(&self, command: Command) -> String {
        match command {
            Command::Ping => RespResponseFormatter::to_simple_str(PONG.to_string()),
            Command::Echo(arg) => RespResponseFormatter::to_bulk_str(arg),
            Command::Get(key) => {
                let store = self.store.lock().unwrap();

                match store.get(&key) {
                    Some(val) => RespResponseFormatter::to_bulk_str(val.to_owned()),
                    None => RESP_NULL_BULK_STR.to_string(),
                }
            }
            Command::Set((key, val)) => {
                let mut store = self.store.lock().unwrap();

                store.insert(key, val);

                RespResponseFormatter::to_simple_str(OK.to_string())
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
