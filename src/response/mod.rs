use crate::{
    command::Command,
    persist::store::StoreService,
    resp::constant::{RESP_NULL_BULK_STR, RESP_TERMINATOR},
};

const PONG: &str = "PONG";
const OK: &str = "OK";
const GENERIC_ERR: &str = "Unknown error";

#[derive(Debug, Clone)]
pub struct ResponseService {
    store: StoreService,
}

#[allow(unused)]
impl ResponseService {
    pub fn new(store: StoreService) -> Self {
        Self { store }
    }

    // Ok(value.and(RespResponseFormatter::to_bulk_str(value)).or(
    //     RespResponseFormatter::to_bulk_str(RESP_NULL_BULK_STR.to_string()),
    // ))

    // let inner_result = match value {
    //     Some(v) => RespResponseFormatter::to_bulk_str(v),
    //     None => RESP_NULL_BULK_STR.to_string(),
    // };

    pub fn execute(&self, command: Command) -> String {
        let result = match command {
            Command::Ping => Ok(Formatter::to_simple_str(PONG.to_string())),
            Command::Echo(arg) => Ok(Formatter::to_bulk_str(arg)),
            Command::Get(key) => self.store.get(key).and_then(|value| {
                Ok(value
                    .and_then(|v| Some(Formatter::to_bulk_str(v)))
                    .unwrap_or(RESP_NULL_BULK_STR.to_string()))
            }),
            Command::Set((key, val)) => self
                .store
                .set(key, val)
                .and(Ok(Formatter::to_simple_str(OK.to_string()))),
            Command::SetWithExp {
                kv: (key, val),
                ttl,
                exp_type,
                options,
            } => self
                .store
                .set_exp(key, val, ttl, exp_type, options)
                .and(Ok(Formatter::to_simple_str(OK.to_string()))),
            Command::Info(arg) => {
                println!("info arg: {:?}", arg);

                Ok(Formatter::to_bulk_str("role:master".to_string()))
            }
        };

        result.unwrap_or(Formatter::to_err_str(GENERIC_ERR.to_string()))
    }
}

struct Formatter;

impl Formatter {
    fn to_bulk_str(value: String) -> String {
        format!(
            "${len}{terminator}{val}{terminator}",
            len = value.len(),
            val = value,
            terminator = RESP_TERMINATOR
        )
    }

    fn to_simple_str(value: String) -> String {
        format!(
            "+{val}{terminator}",
            val = value,
            terminator = RESP_TERMINATOR
        )
    }

    fn to_err_str(value: String) -> String {
        format!(
            "-{val}{terminator}",
            val = value,
            terminator = RESP_TERMINATOR
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Formatter;

    #[test]
    fn simple_str() {
        let input = "OK";
        let result = Formatter::to_simple_str(input.to_string());

        assert_eq!(result, "+OK\r\n");
    }

    #[test]
    fn bulk_str() {
        let input = "blueberry";
        let result = Formatter::to_bulk_str(input.to_string());

        assert_eq!(result, "$9\r\nblueberry\r\n");
    }

    #[test]
    fn error_str() {
        let input = "Unknown err";
        let result = Formatter::to_err_str(input.to_string());

        assert_eq!(result, "-Unknown err\r\n");
    }
}
