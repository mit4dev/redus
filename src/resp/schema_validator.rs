use regex::Regex;

struct BulkStringValidator;

trait ValidatorSchemaProvider {
    fn get_schema() -> Regex;
}

impl ValidatorSchemaProvider for BulkStringValidator {
    fn get_schema() -> Regex {
        Regex::new(r#"^\$([0-9]+)\r\n(.+)\r\n$"#).unwrap()
    }
}
