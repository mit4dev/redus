use super::constant::RESP_TERMINATOR;

pub type Tokens = Vec<String>;

#[derive(Debug)]
pub struct RespTokens(pub Tokens);

impl TryFrom<String> for RespTokens {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        let result = match value.contains(RESP_TERMINATOR) {
            true => {
                let splitted: Tokens = value.split(RESP_TERMINATOR).map(|s| s.to_owned()).collect();

                splitted
            }
            _ => vec![value],
        };

        Ok(RespTokens(result))
    }
}

impl From<Vec<String>> for RespTokens {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl Into<String> for RespTokens {
    fn into(self) -> String {
        self.0.join(RESP_TERMINATOR)
    }
}

#[cfg(test)]
mod tests {
    use super::RespTokens;

    #[test]
    fn tokenize_simple_string() {
        let input = "+HelloWorld\r\n".to_string();
        let tokens = RespTokens::try_from(input).unwrap().0;

        assert_eq!(tokens, vec!["+HelloWorld", ""])
    }

    #[test]
    fn tokenize_simple_array() {
        let input = "*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n".to_string();
        let tokens = RespTokens::try_from(input).unwrap().0;

        assert_eq!(tokens, vec!["*2", "$5", "hello", "$5", "world", ""])
    }

    #[test]
    fn tokenize_mixed_array() {
        let input = "*3\r\n$5\r\nhello\r\n$5\r\nworld\r\n:1\r\n".to_string();
        let tokens = RespTokens::try_from(input).unwrap().0;

        assert_eq!(tokens, vec!["*3", "$5", "hello", "$5", "world", ":1", ""])
    }

    #[test]
    fn tokenize_complex_array() {
        let input = "*4\r\n$5\r\nhello\r\n$5\r\nworld\r\n:1\r\n*1\r\n+simple\r\n".to_string();
        let tokens = RespTokens::try_from(input).unwrap().0;

        assert_eq!(
            tokens,
            vec!["*4", "$5", "hello", "$5", "world", ":1", "*1", "+simple", ""]
        )
    }
}
