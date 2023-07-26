pub enum Command {
    Test
}

impl From<String> for Command {
    fn from(value: String) -> Self {
        match value.as_str() {
            "test" => Self::Test,
            _ => panic!("invalid pattern: {value}.")
        }
    }
}
