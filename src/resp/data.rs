use anyhow::{anyhow, Error};

use super::{
    constant::{
        EMPTY_STR, RESP_DEL_ARRAY, RESP_DEL_BULK_STR, RESP_DEL_INTEGER, RESP_DEL_SIMPLE_STR,
    },
    token::RespTokens,
};

#[derive(Debug, PartialEq)]
pub enum RespData {
    BulkString(String),
    SimpleString(String),
    Integer(i32),
    Array(Vec<RespData>),
}

impl TryFrom<Vec<u8>> for RespData {
    type Error = anyhow::Error;

    fn try_from(_value: Vec<u8>) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[allow(unused)]
impl RespData {
    fn into_string(self) -> Option<String> {
        match self {
            Self::SimpleString(s) => Some(s),
            Self::BulkString(s) => Some(s),
            _ => None,
        }
    }

    fn into_arr(self) -> Option<Vec<RespData>> {
        match self {
            Self::Array(vec) => {
                let mut res = Vec::new();

                for v in vec {
                    match v {
                        Self::Array(data) => panic!("dont know"),
                        // Self::SimpleString(str) | Self::BulkString(str) =>
                        _ => res.push(v),
                    }
                }

                Some(res)
            }
            _ => panic!("dont care"),
        }
    }
}

impl Into<Option<String>> for RespData {
    fn into(self) -> Option<String> {
        match self {
            RespData::BulkString(s) | RespData::SimpleString(s) => Some(s),
            _ => None,
        }
    }
}

#[allow(unused)]
enum Raw {
    Str(String),
    Int(i32),
    Vec(Vec<Raw>),
}

#[derive(Debug, PartialEq)]
pub enum Raw2 {
    SimpleString(String),
    BulkString(String),
    Integer(String),
    ArrayFlat(String),
}

impl TryInto<Vec<String>> for Raw2 {
    type Error = Error;

    fn try_into(self) -> Result<Vec<String>, Self::Error> {
        match self {
            Raw2::ArrayFlat(tokenized_str) => {
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

impl TryFrom<RespTokens> for Raw2 {
    type Error = anyhow::Error;

    fn try_from(value: RespTokens) -> Result<Self, Self::Error> {
        let mut iter = value.0.iter();

        let result = match iter.next() {
            Some(type_part) if type_part.starts_with(RESP_DEL_SIMPLE_STR) => Raw2::SimpleString(
                type_part
                    .replace(RESP_DEL_SIMPLE_STR, EMPTY_STR)
                    .to_string(),
            ),
            Some(type_part) if type_part.starts_with(RESP_DEL_BULK_STR) => {
                Raw2::BulkString(iter.next().unwrap().to_string())
            }
            Some(type_part) if type_part.starts_with(RESP_DEL_INTEGER) => {
                Raw2::Integer(type_part.replace(RESP_DEL_INTEGER, EMPTY_STR).to_string())
            }
            Some(type_part) if type_part.starts_with(RESP_DEL_ARRAY) => {
                let mut vec = "".to_string();
                let data: Vec<String> = iter.clone().map(|x| x.to_owned()).collect();
                vec.push_str(data.clone().join(" ").trim_end());

                Raw2::ArrayFlat(vec)
            }
            _ => return Err(anyhow!("Cannot parse data")),
        };

        Ok(result)
    }
}

impl TryInto<Vec<Raw>> for RespData {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<Raw>, Self::Error> {
        match self {
            RespData::Array(items) => {
                let mut raw_items: Vec<Raw> = Vec::new();
                for item in items {
                    match item {
                        RespData::SimpleString(s) | RespData::BulkString(s) => {
                            raw_items.push(Raw::Str(s))
                        }
                        _ => panic!("Didn't handle"),
                    }
                }

                Ok(raw_items)
            }
            _ => Err(anyhow::anyhow!("Cannot convert primitives")),
        }
    }
}

#[allow(unused)]
#[derive(thiserror::Error, Debug)]
pub enum RespDataParseError {
    #[error("Unknown RESP data parse error")]
    Unknown,
}

impl TryFrom<RespTokens> for RespData {
    type Error = RespDataParseError;

    fn try_from(value: RespTokens) -> Result<Self, Self::Error> {
        let mut iter = value.0.iter();

        let result = match iter.next() {
            Some(type_part) if type_part.starts_with(RESP_DEL_SIMPLE_STR) => {
                RespData::SimpleString(type_part.replace(RESP_DEL_SIMPLE_STR, EMPTY_STR).to_owned())
            }
            Some(type_part) if type_part.starts_with(RESP_DEL_BULK_STR) => {
                RespData::BulkString(iter.next().unwrap().to_owned())
            }
            Some(type_part) if type_part.starts_with(RESP_DEL_INTEGER) => RespData::Integer(
                type_part
                    .replace(RESP_DEL_INTEGER, EMPTY_STR)
                    .parse::<i32>()
                    .unwrap(),
            ),
            Some(type_part) if type_part.starts_with(RESP_DEL_ARRAY) => {
                let arr_len = type_part
                    .replace(RESP_DEL_ARRAY, EMPTY_STR)
                    .parse::<usize>()
                    .unwrap();

                // TODO: handle complex arrays
                match arr_len {
                    0 => RespData::Array(vec![]),
                    x @ (1..) => {
                        let mut vec: Vec<RespData> = Vec::new();
                        println!("Array length: {}", x);
                        for i in 0..x {
                            let data: Vec<String> = iter
                                .clone()
                                .skip(i * 2)
                                .take(2)
                                .map(|x| x.to_owned())
                                .collect();
                            let tokens = RespTokens::from(data);
                            println!("tokens: {:?}", tokens);
                            vec.push(RespData::try_from(tokens).unwrap())
                        }

                        RespData::Array(vec)
                    }
                }
            }
            x => panic!("oops {}", x.unwrap()),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::{
        data::{Raw2, RespData},
        token::RespTokens,
    };

    #[test]
    fn raw2_mixed() {
        let input = String::from("$4\r\nPING\r\n+Hello\r\n:22\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = Raw2::try_from(tokens).unwrap();

        assert_eq!(result, Raw2::BulkString("PING".to_string()));
    }

    #[test]
    fn raw2_simple_array() {
        let input = String::from("*3\r\n$4\r\nPING\r\n+Hello\r\n:22\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = Raw2::try_from(tokens).unwrap();

        assert_eq!(result, Raw2::ArrayFlat("$4 PING +Hello :22".to_string()));
    }

    #[test]
    fn raw2_nested_array() {
        let input = String::from("*2\r\n$4\r\nPING\r\n*2\r\n$5\r\nHello\r\n:444\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = Raw2::try_from(tokens).unwrap();

        assert_eq!(
            result,
            Raw2::ArrayFlat("$4 PING *2 $5 Hello :444".to_string())
        );
    }

    #[test]
    fn raw2_into_vec() {
        let input = String::from("*2\r\n$4\r\nPING\r\n$5\r\nHello\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let parsed = Raw2::try_from(tokens).unwrap();
        let result: Vec<String> = Raw2::try_into(parsed).unwrap();

        assert_eq!(result, vec!("PING".to_string(), "Hello".to_string()));
    }

    #[test]
    fn simple_string() {
        let input = String::from("+PING\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = RespData::try_from(tokens).unwrap();

        assert_eq!(
            result.into_string().unwrap(),
            RespData::SimpleString("PING".to_string())
                .into_string()
                .unwrap()
        );
    }

    #[test]
    fn simple_array() {
        let input = String::from("*2\r\n$4\r\nPING\r\n+Hello\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = RespData::try_from(tokens).unwrap();

        println!("res: {:?}", result);

        assert_eq!(
            RespData::Array(vec![
                RespData::BulkString("PING".to_string()),
                RespData::SimpleString("Hello".to_string())
            ]),
            result // RespData::Array(vec![RespData::BulkString("PING".to_string())])
        );
    }

    #[test]
    fn simple_mixed_array() {
        let input = String::from("*3\r\n$4\r\nECHO\r\n$5\r\nirfan\r\n:1");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = RespData::try_from(tokens).unwrap();

        println!("res: {:?}", result);

        assert_eq!(
            RespData::Array(vec![
                RespData::BulkString("ECHO".to_string()),
                RespData::BulkString("irfan".to_string()),
                RespData::Integer(1)
            ]),
            result
        );
    }

    #[ignore = "future nested array"]
    #[test]
    fn simple_complex_array() {
        let input = String::from("*2\r\n*3\r\n+ECHO\r\n$5\r\nirfan\r\n+OK\r\n$4\r\nEsra\r\n");
        let tokens = RespTokens::try_from(input).unwrap();
        let result = RespData::try_from(tokens).unwrap();

        println!("res: {:?}", result);

        assert_eq!(
            true,
            false // RespData::Array(vec![RespData::BulkString("PING".to_string())])
        );
    }
}
