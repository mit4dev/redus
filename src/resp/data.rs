use anyhow::{anyhow, Error};

use super::{
    constant::{
        EMPTY_STR, RESP_DEL_ARRAY, RESP_DEL_BULK_STR, RESP_DEL_INTEGER, RESP_DEL_SIMPLE_STR,
    },
    token::RespTokens,
};

#[derive(Debug, PartialEq)]
pub enum RespData {
    SimpleString(String),
    BulkString(String),
    Integer(String),
    ArrayFlat(String),
}

impl TryInto<Vec<String>> for RespData {
    type Error = Error;

    fn try_into(self) -> Result<Vec<String>, Self::Error> {
        match self {
            RespData::ArrayFlat(tokenized_str) => {
                let mut tokens = tokenized_str.split(" ");

                let mut result: Vec<String> = Vec::new();
                loop {
                    match tokens.next() {
                        Some(s) => {
                            if s.starts_with(RESP_DEL_BULK_STR) {
                                result.push(tokens.next().unwrap_or("").to_string())
                            }

                            if s.starts_with(RESP_DEL_SIMPLE_STR) {
                                result.push(s.replace(RESP_DEL_SIMPLE_STR, EMPTY_STR).to_string())
                            }

                            if s.starts_with(RESP_DEL_INTEGER) {
                                result.push(s.replace(RESP_DEL_INTEGER, EMPTY_STR).to_string())
                            }

                            // TODO: handle nested arrays / maps
                            // if s.starts_with(RESP_DEL_ARRAY) {
                            //     result.push(
                            //         Raw2::try_from(s.to_string()).unwrap().try_into().unwrap(),
                            //     )
                            // }
                        }
                        None => break,
                    }
                }

                Ok(result)
            }
            Self::BulkString(s) | Self::SimpleString(s) | Self::Integer(s) => Ok(vec![s]),
        }
    }
}

impl TryFrom<RespTokens> for RespData {
    type Error = anyhow::Error;

    fn try_from(value: RespTokens) -> Result<Self, Self::Error> {
        let mut iter = value.0.iter();

        let result = match iter.next() {
            Some(type_part) if type_part.starts_with(RESP_DEL_SIMPLE_STR) => {
                RespData::SimpleString(
                    type_part
                        .replace(RESP_DEL_SIMPLE_STR, EMPTY_STR)
                        .to_string(),
                )
            }
            Some(type_part) if type_part.starts_with(RESP_DEL_BULK_STR) => {
                RespData::BulkString(iter.next().unwrap().to_string())
            }
            Some(type_part) if type_part.starts_with(RESP_DEL_INTEGER) => {
                RespData::Integer(type_part.replace(RESP_DEL_INTEGER, EMPTY_STR).to_string())
            }
            Some(type_part) if type_part.starts_with(RESP_DEL_ARRAY) => {
                let mut vec = "".to_string();
                let data: Vec<String> = iter.clone().map(|x| x.to_owned()).collect();
                vec.push_str(data.clone().join(" ").trim_end());

                RespData::ArrayFlat(vec)
            }
            _ => return Err(anyhow!("Cannot parse data")),
        };

        Ok(result)
    }
}

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum RespDataParseError {
    #[error("Unknown RESP data parse error")]
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::resp::{data::RespData, token::RespTokens};

    #[test]
    fn mixed() {
        let input = String::from("$4\r\nPING\r\n+Hello\r\n:22\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = RespData::try_from(tokens).unwrap();

        assert_eq!(result, RespData::BulkString("PING".to_string()));
    }

    #[test]
    fn simple_array() {
        let input = String::from("*3\r\n$4\r\nPING\r\n+Hello\r\n:22\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = RespData::try_from(tokens).unwrap();

        assert_eq!(
            result,
            RespData::ArrayFlat("$4 PING +Hello :22".to_string())
        );
    }

    #[test]
    fn nested_array() {
        let input = String::from("*2\r\n$4\r\nPING\r\n*2\r\n$5\r\nHello\r\n:444\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = RespData::try_from(tokens).unwrap();

        assert_eq!(
            result,
            RespData::ArrayFlat("$4 PING *2 $5 Hello :444".to_string())
        );
    }

    #[test]
    fn into_vec() {
        let input = String::from("*2\r\n$4\r\nPING\r\n$5\r\nHello\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let parsed = RespData::try_from(tokens).unwrap();
        let result: Vec<String> = RespData::try_into(parsed).unwrap();

        assert_eq!(result, vec!("PING".to_string(), "Hello".to_string()));
    }
}
