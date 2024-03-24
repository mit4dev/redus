use super::constant::RESP_DEL_BULK_STR;

pub trait TokenCounter {
    fn count(tokens: Vec<String>) -> u32;
}

pub struct BulkStringTokenCounter;

impl TokenCounter for BulkStringTokenCounter {
    fn count(tokens: Vec<String>) -> u32 {
        let mut count = 0u32;

        let mut iter = tokens.iter();
        loop {
            match iter.next() {
                Some(token) => {
                    if token.starts_with(RESP_DEL_BULK_STR) {
                        count += 1;
                        iter.next();
                    }
                }
                None => break,
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::util::{BulkStringTokenCounter, TokenCounter};

    #[test]
    fn count() {
        let tokens = vec![
            "$4", "home", ":0", "+Hello", "$3", "foo", "$0", "", "$4", "$$$$",
        ];
        let result =
            BulkStringTokenCounter::count(tokens.into_iter().map(|t| t.to_string()).collect());

        assert_eq!(result, 4)
    }
}
