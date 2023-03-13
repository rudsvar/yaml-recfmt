//! A formatter for nested YAML files.
//!
//! YAML can contain multiline strings that are also YAML,
//! but normal formatters will (understandably) not format
//! these nested values.
//!
//! This application does it anyway, which can be useful
//! for files such as Helm charts with configuration in them.
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
//! Read from one file and write to another.
//!
//! ```bash
//! yaml-recfmt input.yaml --output output.yaml
//! ```
//!
//! Pipe through `yaml-recfmt`.
//!
//! ```bash
//! cat input.yaml | yaml-recfmt > output.yaml
//! ```

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

/// Recursively format a YAML-formatted string.
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
pub fn format(yaml: &str) -> serde_yaml::Result<String> {
    let parsed: Value = serde_yaml::from_str(yaml)?;
    let formatted = format_value(parsed);
    serde_yaml::to_string(&formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_change_normal_yaml() {
        let input = r#"foo: bar
"#;
        let expected = format(input).unwrap();
        assert_eq!(input, expected)
    }

    #[test]
    fn unnecessary_whitespace_is_removed() {
        let input = r#"foo:    bar
"#;
        let expected = r#"foo: bar
"#;
        let output = format(input).unwrap();
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
        let output = format(input).unwrap();
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
        let output = format(input).unwrap();
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
        let output = format(input).unwrap();
        assert_eq!(expected, output)
    }
}
