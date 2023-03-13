//! A formatter for YAML files.
//!
//! YAML can contain multiline strings that are also YAML,
//! but normal formatters will (understandably) not format
//! these nested values.
//!
//! When the `--recursive` flag is provided, this application does so anyway.
//! Note that this actually changes the data of the string, so use with care.
//!
//! # Installation
//!
//! ```bash
//! cargo install --git https://github.com/rudsvar/yaml-recfmt
//! ```
//!
//! # Examples
//!
//! Display help.
//!
//! ```bash
//! yaml-recfmt --help
//! ```
//!
//! Pipe through `yaml-recfmt`.
//!
//! ```bash
//! cat input.yaml | yaml-recfmt > output.yaml
//! ```
//!
//! Format a set of files recursively in-place.
//!
//! ```bash
//! yaml-recfmt --in-place --recursive examples/*.yaml
//! ```

use regex::Regex;
use serde_yaml::Value;

/// Recursively formats strings found within a YAML value.
pub fn format_value(value: Value) -> Value {
    match value {
        Value::String(s) => match serde_yaml::from_str::<Value>(&s) {
            // Inner value is more yaml, recurse
            Ok(v) if v.is_mapping() || v.is_sequence() => {
                let formatted = serde_yaml::to_string(&format_value(v));
                Value::String(formatted.expect("failed to serialize yaml"))
            }
            // Not interesting value
            Ok(v) => v,
            // Not yaml, keep original
            Err(_) => Value::String(s),
        },
        // Format each element of the sequence
        Value::Sequence(s) => Value::Sequence(s.into_iter().map(format_value).collect()),
        // Format the values of the mapping
        Value::Mapping(m) => {
            Value::Mapping(m.into_iter().map(|(k, v)| (k, format_value(v))).collect())
        }
        // Keep all other values
        value => value,
    }
}

/// Recursively formats a YAML-formatted string.
///
/// # Examples
///
/// ```
/// assert_eq!("foo: bar\n", yaml_recfmt::format("foo: bar").unwrap());
/// ```
///
/// ```
/// assert_eq!("foo: bar\n", yaml_recfmt::format("foo:   bar").unwrap());
/// ```
pub fn format_recursive(yaml: &str) -> serde_yaml::Result<String> {
    let parsed: Value = serde_yaml::from_str(yaml)?;
    let value = format_value(parsed);
    let formatted = serde_yaml::to_string(&value)?;
    let re = Regex::new("(\\s+)(0\\d+)").unwrap();
    Ok(re.replace_all(&formatted, "$1'$2'").to_string())
}

/// Formats a YAML-formatted string.
pub fn format(yaml: &str) -> serde_yaml::Result<String> {
    let parsed: Value = serde_yaml::from_str(yaml)?;
    let formatted = serde_yaml::to_string(&parsed)?;
    let re = Regex::new("(\\s+)(0\\d+)").unwrap();
    Ok(re.replace_all(&formatted, "$1'$2'").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_change_normal_yaml() {
        let input = r#"foo: bar
"#;
        let expected = format_recursive(input).unwrap();
        assert_eq!(input, expected)
    }

    #[test]
    fn unnecessary_whitespace_is_removed() {
        let input = r#"foo:    bar
"#;
        let expected = r#"foo: bar
"#;
        let output = format_recursive(input).unwrap();
        assert_eq!(expected, output)
    }

    #[test]
    fn mapping_fields_are_deindented() {
        let input = r#"foo:
        bar: 123
        baz: 345
"#;
        let expected = r#"foo:
  bar: 123
  baz: 345
"#;
        let output = format_recursive(input).unwrap();
        assert_eq!(expected, output)
    }

    #[test]
    fn nested_yaml_gets_formatted() {
        let input = r#"foo: |
    bar:
        baz: 345
"#;
        let expected = r#"foo: |
  bar:
    baz: 345
"#;
        let output = format_recursive(input).unwrap();
        assert_eq!(expected, output)
    }

    #[test]
    fn sequences_are_formatted() {
        let input = r#"sequence: |
            - foo: 1
              bar:
              - a
              - b
            - foo: 3
              bar: 4
"#;
        let expected = r#"sequence: |
  - foo: 1
    bar:
    - a
    - b
  - foo: 3
    bar: 4
"#;
        let output = format_recursive(input).unwrap();
        assert_eq!(expected, output)
    }

    #[test]
    fn quotes_are_kept_if_zero_prefix() {
        let input = r#"foo:
  bar: '123'
  baz: '0123'
"#;
        let expected = r#"foo:
  bar: 123
  baz: '0123'
"#;
        let output = format_recursive(input).unwrap();
        assert_eq!(expected, output)
    }

    #[test]
    fn quotes_are_kept_if_zero_prefix_when_nested() {
        let input = r#"foo: |
  bar: '123'
  baz: '0123'
"#;
        let expected = r#"foo: |
  bar: 123
  baz: '0123'
"#;
        let output = format_recursive(input).unwrap();
        assert_eq!(expected, output)
    }

    #[test]
    fn quotes_are_kept_if_zero_prefix_in_sequence() {
        let input = r#"foo:
  - '123'
  - '0123'
"#;
        let expected = r#"foo:
- 123
- '0123'
"#;
        let output = format_recursive(input).unwrap();
        assert_eq!(expected, output)
    }
}
