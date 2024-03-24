use anyhow::Error;

const ECHO: &str = "echo";
const PING: &str = "ping";
const SET: &str = "set";
const GET: &str = "get";

#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Echo(String),
    Set((String, String)),
    Get(String),
}

impl TryFrom<&str> for Command {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            PING => Ok(Command::Ping),
            _ => Err(anyhow::anyhow!("Cannot parse `Command` from &str")),
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
            [cmd, key, val] if *cmd.to_lowercase() == SET.to_string() => {
                Ok(Command::Set((key.to_string(), val.to_string())))
            }
            [cmd, key] if *cmd.to_lowercase() == GET.to_string() => {
                Ok(Command::Get(key.to_string()))
            }
            _ => Err(anyhow::anyhow!("Cannot parse `Command`")),
        }
    }
}

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

    #[test]
    fn set() {
        let input = vec!["set".to_string(), "key".to_string(), "val".to_string()];
        let result = Command::try_from(input).unwrap();

        assert_eq!(result, Command::Set(("key".to_string(), "val".to_string())));
    }

    #[test]
    fn get() {
        let key = "key";
        let input: Vec<String> = vec!["get".to_string(), key.to_string()];
        let result = Command::try_from(input).unwrap();

        assert_eq!(result, Command::Get(key.to_string()));
    }
}
