#[derive(Debug)]
pub enum DataType {
    SimpleString,
    SimpleError,
    Integer,
    BulkString,
    Array,
}

#[derive(thiserror::Error, Debug)]
pub enum DataTypeParsingError {
    #[error("Unknown type_flag: {0}")]
    UnknownTypeFlag(char),
}

impl TryFrom<char> for DataType {
    type Error = DataTypeParsingError;

    fn try_from(value: char) -> anyhow::Result<Self, Self::Error> {
        match value {
            '+' => Ok(DataType::SimpleString),
            '$' => Ok(DataType::BulkString),
            '-' => Ok(DataType::SimpleError),
            ':' => Ok(DataType::Integer),
            '*' => Ok(DataType::Array),
            _ => Err(DataTypeParsingError::UnknownTypeFlag(value)),
        }
    }
}
