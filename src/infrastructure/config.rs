use std::collections::HashMap;

const PREFIX: &str = r#"BOT_"#;

const ERR_REQUIRED_KEY: &str = r#"required config key"#;
const ERR_PARSE_KEY: &str = r#"failed to parse config key"#;

#[derive(Clone)]
pub struct Config {
    kv: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let kv = std::env::vars()
            .filter(|(k, _)| k.starts_with(PREFIX))
            .collect();
        Self { kv }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn optional(&self, key: &str) -> Option<&str> {
        self.kv.get(key).map(|s| s.as_str())
    }

    pub fn required(&self, key: &str) -> anyhow::Result<&str> {
        self.optional(key)
            .ok_or_else(|| anyhow::anyhow!("{ERR_REQUIRED_KEY} '{key}'"))
    }

    pub fn parse_optional<T>(&self, key: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.optional(key).and_then(|s| s.parse::<T>().ok())
    }

    pub fn parse_required<T>(&self, key: &str) -> anyhow::Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let s = self.required(key)?;
        s.parse()
            .map_err(|e| anyhow::anyhow!("{ERR_PARSE_KEY} '{key}': {e}"))
    }
}
