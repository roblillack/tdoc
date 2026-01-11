//! Document metadata parsing and serialization.
//!
//! This module provides utilities for handling document metadata, such as
//! YAML frontmatter in Markdown files. Metadata is typically stored at the
//! beginning of a document enclosed in `---` delimiters.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A value that can appear in document metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl Value {
    /// Returns `true` if this value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Returns the value as a string, if it is one.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the value as a boolean, if it is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the value as an integer, if it is one.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the value as a float, if it is one.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Returns the value as an array, if it is one.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns the value as an object, if it is one.
    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(i)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

/// Ordered map of document metadata key-value pairs.
///
/// Uses [`IndexMap`] to preserve insertion order, which is important for
/// round-tripping documents without reordering metadata fields.
pub type Metadata = IndexMap<String, Value>;

/// Extracts YAML metadata (frontmatter) from the beginning of a string.
///
/// Metadata must start with `---` on the first line and end with `---`
/// on its own line. Returns the parsed metadata and the remaining content.
///
/// # Examples
///
/// ```
/// use tdoc::metadata::extract;
///
/// let input = "---\ntitle: Hello\n---\n\nContent here";
/// let (metadata, content) = extract(input).unwrap();
///
/// assert!(metadata.is_some());
/// let meta = metadata.unwrap();
/// assert_eq!(meta.get("title").unwrap().as_str(), Some("Hello"));
/// assert_eq!(content, "\nContent here");
/// ```
pub fn extract(input: &str) -> crate::Result<(Option<Metadata>, &str)> {
    // Metadata must start at the very beginning with ---
    if !input.starts_with("---") {
        return Ok((None, input));
    }

    // Find the end of the first line (the opening ---)
    let after_opening = match input[3..].find('\n') {
        Some(pos) => 3 + pos + 1,
        None => return Ok((None, input)), // No newline after opening ---
    };

    // Check that the opening line is just --- (possibly with trailing whitespace)
    let opening_line = input[3..after_opening - 1].trim();
    if !opening_line.is_empty() {
        return Ok((None, input));
    }

    // Find the closing ---
    let rest = &input[after_opening..];
    let closing_pos = find_closing_delimiter(rest)?;

    match closing_pos {
        Some(pos) => {
            let yaml_content = &rest[..pos];
            let after_closing = after_opening + pos + 3; // Skip the closing ---

            // Skip the newline after closing --- if present
            let content_start = if input[after_closing..].starts_with('\n') {
                after_closing + 1
            } else if input[after_closing..].starts_with("\r\n") {
                after_closing + 2
            } else {
                after_closing
            };

            let metadata = parse_yaml(yaml_content)?;
            Ok((Some(metadata), &input[content_start..]))
        }
        None => Ok((None, input)),
    }
}

/// Finds the position of the closing `---` delimiter.
fn find_closing_delimiter(s: &str) -> crate::Result<Option<usize>> {
    let mut pos = 0;
    for line in s.lines() {
        if line.trim() == "---" {
            return Ok(Some(pos));
        }
        pos += line.len() + 1; // +1 for newline
    }
    Ok(None)
}

/// Parses YAML content into a Metadata map.
fn parse_yaml(yaml: &str) -> crate::Result<Metadata> {
    if yaml.trim().is_empty() {
        return Ok(Metadata::new());
    }

    let value: serde_yaml::Value = serde_yaml::from_str(yaml)?;

    match value {
        serde_yaml::Value::Mapping(map) => {
            let mut result = Metadata::new();
            for (k, v) in map {
                if let serde_yaml::Value::String(key) = k {
                    result.insert(key, yaml_to_value(v));
                }
            }
            Ok(result)
        }
        serde_yaml::Value::Null => Ok(Metadata::new()),
        _ => Err("Metadata must be a YAML mapping".into()),
    }
}

/// Converts a serde_yaml::Value to our Value type.
fn yaml_to_value(v: serde_yaml::Value) -> Value {
    match v {
        serde_yaml::Value::Null => Value::Null,
        serde_yaml::Value::Bool(b) => Value::Bool(b),
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::Null
            }
        }
        serde_yaml::Value::String(s) => Value::String(s),
        serde_yaml::Value::Sequence(seq) => {
            Value::Array(seq.into_iter().map(yaml_to_value).collect())
        }
        serde_yaml::Value::Mapping(map) => {
            let mut result = IndexMap::new();
            for (k, v) in map {
                if let serde_yaml::Value::String(key) = k {
                    result.insert(key, yaml_to_value(v));
                }
            }
            Value::Object(result)
        }
        serde_yaml::Value::Tagged(tagged) => yaml_to_value(tagged.value),
    }
}

/// Serializes metadata to YAML format with delimiters.
///
/// # Examples
///
/// ```
/// use tdoc::metadata::{serialize, Metadata, Value};
///
/// let mut meta = Metadata::new();
/// meta.insert("title".to_string(), Value::String("Hello".to_string()));
///
/// let yaml = serialize(&meta).unwrap();
/// assert!(yaml.starts_with("---\n"));
/// assert!(yaml.contains("title: Hello"));
/// assert!(yaml.ends_with("---\n"));
/// ```
pub fn serialize(metadata: &Metadata) -> crate::Result<String> {
    if metadata.is_empty() {
        return Ok(String::new());
    }

    let yaml_value = value_to_yaml_mapping(metadata);
    let yaml_str = serde_yaml::to_string(&yaml_value)?;

    Ok(format!("---\n{}---\n", yaml_str))
}

/// Converts our Metadata to a serde_yaml::Value for serialization.
fn value_to_yaml_mapping(metadata: &Metadata) -> serde_yaml::Value {
    let mut map = serde_yaml::Mapping::new();
    for (k, v) in metadata {
        map.insert(
            serde_yaml::Value::String(k.clone()),
            value_to_yaml(v.clone()),
        );
    }
    serde_yaml::Value::Mapping(map)
}

/// Converts our Value type to serde_yaml::Value.
fn value_to_yaml(v: Value) -> serde_yaml::Value {
    match v {
        Value::Null => serde_yaml::Value::Null,
        Value::Bool(b) => serde_yaml::Value::Bool(b),
        Value::Integer(i) => serde_yaml::Value::Number(i.into()),
        Value::Float(f) => serde_yaml::Value::Number(serde_yaml::Number::from(f)),
        Value::String(s) => serde_yaml::Value::String(s),
        Value::Array(arr) => {
            serde_yaml::Value::Sequence(arr.into_iter().map(value_to_yaml).collect())
        }
        Value::Object(obj) => {
            let mut map = serde_yaml::Mapping::new();
            for (k, v) in obj {
                map.insert(serde_yaml::Value::String(k), value_to_yaml(v));
            }
            serde_yaml::Value::Mapping(map)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_metadata() {
        let input = "---\ntitle: Hello World\nauthor: Test\n---\n\nContent here";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_some());
        let meta = meta.unwrap();
        assert_eq!(meta.get("title").unwrap().as_str(), Some("Hello World"));
        assert_eq!(meta.get("author").unwrap().as_str(), Some("Test"));
        assert_eq!(content, "\nContent here");
    }

    #[test]
    fn test_extract_no_metadata() {
        let input = "Just some content\nwithout metadata";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_none());
        assert_eq!(content, input);
    }

    #[test]
    fn test_extract_empty_metadata() {
        let input = "---\n---\n\nContent";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_some());
        assert!(meta.unwrap().is_empty());
        assert_eq!(content, "\nContent");
    }

    #[test]
    fn test_extract_metadata_with_arrays() {
        let input = "---\ntags:\n  - rust\n  - programming\n---\n\nContent";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_some());
        let meta = meta.unwrap();
        let tags = meta.get("tags").unwrap().as_array().unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].as_str(), Some("rust"));
        assert_eq!(tags[1].as_str(), Some("programming"));
        assert_eq!(content, "\nContent");
    }

    #[test]
    fn test_extract_metadata_with_nested_object() {
        let input = "---\nauthor:\n  name: John\n  email: john@example.com\n---\n\nContent";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_some());
        let meta = meta.unwrap();
        let author = meta.get("author").unwrap().as_object().unwrap();
        assert_eq!(author.get("name").unwrap().as_str(), Some("John"));
        assert_eq!(
            author.get("email").unwrap().as_str(),
            Some("john@example.com")
        );
        assert_eq!(content, "\nContent");
    }

    #[test]
    fn test_serialize_metadata() {
        let mut meta = Metadata::new();
        meta.insert("title".to_string(), Value::String("Test".to_string()));
        meta.insert("draft".to_string(), Value::Bool(true));

        let yaml = serialize(&meta).unwrap();
        assert!(yaml.starts_with("---\n"));
        assert!(yaml.ends_with("---\n"));
        assert!(yaml.contains("title: Test"));
        assert!(yaml.contains("draft: true"));
    }

    #[test]
    fn test_serialize_empty_metadata() {
        let meta = Metadata::new();
        let yaml = serialize(&meta).unwrap();
        assert!(yaml.is_empty());
    }

    #[test]
    fn test_roundtrip() {
        let input = "---\ntitle: Hello World\ncount: 42\ndraft: false\n---\n\nContent";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_some());
        let meta = meta.unwrap();

        let serialized = serialize(&meta).unwrap();
        let (meta2, _) = extract(&format!("{}{}", serialized, content)).unwrap();

        assert!(meta2.is_some());
        let meta2 = meta2.unwrap();

        assert_eq!(meta.get("title"), meta2.get("title"));
        assert_eq!(meta.get("count"), meta2.get("count"));
        assert_eq!(meta.get("draft"), meta2.get("draft"));
    }

    #[test]
    fn test_not_metadata_if_not_at_start() {
        let input = "Some content\n---\ntitle: Hello\n---\n";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_none());
        assert_eq!(content, input);
    }

    #[test]
    fn test_metadata_with_dashes_in_content() {
        let input = "---\ntitle: Test\n---\n\nContent with --- dashes";
        let (meta, content) = extract(input).unwrap();

        assert!(meta.is_some());
        assert_eq!(content, "\nContent with --- dashes");
    }
}
