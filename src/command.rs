use anyhow::Error;

const ECHO: &str = "echo";
#[allow(unused)]
const PONG: &str = "pong";
const PING: &str = "ping";

#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Echo(String),
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            PING => Ok(Command::Ping),
            _ => Err(anyhow::anyhow!("Invalid command")),
        }
    }
}

impl TryFrom<Vec<String>> for Command {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        match value.as_slice() {
            [command] if *command.to_lowercase() == PING.to_string() => Ok(Command::Ping),
            [command, arg] if *command.to_lowercase() == ECHO.to_string() => {
                Ok(Command::Echo(arg.to_string()))
            }
            _ => Err(anyhow::anyhow!("Invalid command")),
        }
    }
}

impl TryInto<String> for Command {
    type Error = Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Command::Ping => Ok("+PONG\r\n".to_string()),
            Command::Echo(s) => Ok(format!("${0}\r\n{1}\r\n", s.len(), s)),
        }
    }
}

// impl TryFrom<RespData> for Command {
//     type Error = Error;

//     fn try_from(value: RespData) -> Result<Self, Self::Error> {
//         match value {
//             RespData::Array(a) => match &a[..] {
//                 [a] => match a {
//                     RespData::BulkString(s1) if s1.clone().to_lowercase() == PING.to_string() => {
//                         Ok(Command::Ping)
//                     }
//                     _ => panic!("one liner"),
//                 },
//                 [a, b] => match (a, b) {
//                     (RespData::BulkString(s1), RespData::BulkString(s2), ..)
//                         if s1.to_lowercase().clone() == ECHO.to_string() =>
//                     {
//                         Ok(Command::Echo(s2.clone()))
//                     }

//                     _ => panic!("str {0:?}, {1:?}", a, b),
//                 },
//                 _ => panic!(""),
//             },
//             _ => panic!("val {:?}", value),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::command::Command;

    #[test]
    fn ping() {
        let input = vec!["ping".to_string()];
        let result = Command::try_from(input).unwrap();

        assert_eq!(result, Command::Ping);
    }

    #[test]
    fn echo() {
        let input = vec!["echo".to_string(), "hey".to_string()];
        let result = Command::try_from(input).unwrap();

        assert_eq!(result, Command::Echo("hey".to_string()));
    }

    // #[test]
    // fn resp_ping() {
    //     let input = RespData::Array(vec![RespData::BulkString("ping".to_string())]);
    //     let result = Command::try_from(input).unwrap();

    //     assert_eq!(result, Command::Ping);
    // }

    // #[test]
    // fn resp_simple_echo() {
    //     let echo_word = "blueberry";
    //     let input = RespData::Array(vec![
    //         RespData::BulkString(ECHO.to_string()),
    //         RespData::BulkString(echo_word.to_string()),
    //     ]);
    //     let result = Command::try_from(input).unwrap();

    //     assert_eq!(result, Command::Echo(echo_word.to_string()));
    // }
}
