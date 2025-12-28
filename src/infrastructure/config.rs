use std::collections::HashMap;

const PREFIX: &str = "BOT_";

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
            .ok_or_else(|| anyhow::anyhow!("required config key '{key}'"))
    }

    pub fn parse_optional<T>(&self, key: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.optional(key).and_then(|s| s.parse::<T>().ok())
    }

    pub fn parse_list<T>(&self, key: &str) -> Vec<T>
    where
        T: std::str::FromStr,
    {
        self.optional(key)
            .map(|s| {
                s.split(',')
                    .filter_map(|item| item.trim().parse::<T>().ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn required_list<T>(&self, key: &str) -> anyhow::Result<Vec<T>>
    where
        T: std::str::FromStr,
    {
        let list = self.parse_list(key);
        if list.is_empty() {
            anyhow::bail!("required config key '{key}' is missing or empty");
        }
        Ok(list)
    }
}
