pub type JSON = serde_json::Map<String, serde_json::Value>;

pub trait GetOrDefault<T> {
    fn get_or_default(&self, key: &str) -> T;
}

impl GetOrDefault<i64> for JSON {
    #[inline]
    fn get_or_default(&self, key: &str) -> i64 {
        self.get(key)
            .unwrap_or(&serde_json::Value::Null)
            .as_i64()
            .unwrap_or_default()
    }
}

impl GetOrDefault<String> for JSON {
    #[inline]
    fn get_or_default(&self, key: &str) -> String {
        self.get(key)
            .unwrap_or(&serde_json::Value::Null)
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }
}
