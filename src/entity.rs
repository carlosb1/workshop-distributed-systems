/// Entity domain code, it defines the necessary
/// relationship in the code.
use std::str::FromStr;

/// Node network structure representaiton.
#[derive(Clone)]
pub struct Node {
    pub address: String,
}

impl Node {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT").unwrap_or("3042".to_string());
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let address = format!("{:}:{:}", host, port);
        Node { address }
    }
}
impl Default for Node {
    fn default() -> Self {
        Node {
            address: "0.0.0.0:3042".to_string(),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct VersionedValue {
    pub key: String,
    pub time: u64,
    versioned_value: String,
}

impl VersionedValue {
    pub fn new(key: &str, time: u64) -> Self {
        let versioned_value = format!("{:}@{:}", key.to_string(), time);
        VersionedValue {
            key: key.to_string(),
            time,
            versioned_value,
        }
    }
}
impl ToString for VersionedValue {
    fn to_string(&self) -> String {
        self.versioned_value.clone()
    }
}
#[derive(Debug)]
pub struct ParseVersionedError {}

impl FromStr for VersionedValue {
    type Err = ParseVersionedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split("@");
        let key = splitted.next().ok_or(ParseVersionedError {})?;
        let time = splitted.next().ok_or(ParseVersionedError {})?;
        Ok(VersionedValue {
            key: key.to_string(),
            time: time.parse::<u64>().map_err(|_| ParseVersionedError {})?,
            versioned_value: s.to_string(),
        })
    }
}

#[derive(Clone)]
pub struct SerializedValue {
    value: String,
}

impl ToString for SerializedValue {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl SerializedValue {
    pub fn new(value: String) -> Self {
        SerializedValue { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_versioned_value_to_string() {
        let versioned_value = VersionedValue::new("alice", 1);
        assert_eq!(versioned_value.to_string(), "alice@1".to_string())
    }

    #[test]
    pub fn test_versioned_value_from_str() {
        let versioned_value = VersionedValue::from_str("alice@1").unwrap();
        assert_eq!(versioned_value.versioned_value, "alice@1");
        assert_eq!(versioned_value.key, "alice".to_string());
        assert_eq!(versioned_value.time, 1);
    }

    #[test]
    pub fn test_serialized_value_to_string() {
        let serialized_value = SerializedValue::new("1".to_string());
        assert_eq!(serialized_value.to_string(), "1".to_string());
    }
}
