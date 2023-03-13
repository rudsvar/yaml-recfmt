//! A formatter for nested YAML files.
//!
//! YAML can contain multiline strings that are also YAML,
//! but normal formatters will (understandably) not format
//! these nested values.
//!
//! This application does it anyway, which can be useful
//! for files such as Helm charts with configuration in them.
//!
//! # Examples
//!
//! Display help.
//!
//! ```bash
//! yaml-recfmt -h
//! ```
//!
//! Pipe YAML from one file to another.
//!
//! ```bash
//! cat input.yaml | yaml-recfmt > output.yaml
//! ```
//!
//! Read from one file and write to another.
//!
//! ```bash
//! yaml-recfmt --input input.yaml --output output.yaml
//! ```
//!
//! Format a file in-place.
//!
//! ```bash
//! yaml-recfmt input.yaml
//! ```

use serde_yaml::Value;
use std::io::Write;

/// Parses and formats nested strings found in the YAML value.
pub fn format(value: Value) -> Value {
    match value {
        Value::String(s) => match serde_yaml::from_str::<Value>(&s) {
            // Inner value is more yaml, recurse
            Ok(v) if v.is_mapping() || v.is_sequence() => {
                let formatted = serde_yaml::to_string(&format(v));
                Value::String(formatted.expect("failed to serialize yaml"))
            }
            // Not interesting value
            Ok(v) => v,
            // Not yaml, keep original
            Err(_) => Value::String(s),
        },
        // Format each element of the sequence
        Value::Sequence(s) => Value::Sequence(s.into_iter().map(format).collect()),
        // Format the values of the mapping
        Value::Mapping(m) => Value::Mapping(m.into_iter().map(|(k, v)| (k, format(v))).collect()),
        // Keep all other values
        value => value,
    }
}

/// Read YAML from the reader and write the formatted output to the writer.
pub fn run_format<W: Write>(input: &str, output: W) -> color_eyre::Result<()> {
    // Run formatter on input
    let yaml: Value = serde_yaml::from_str(input)?;
    let formatted = format(yaml);

    // Write formatted value to output
    serde_yaml::to_writer(output, &formatted)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_string(yaml: &str) -> color_eyre::Result<String> {
        let mut output = Vec::new();
        run_format(yaml, &mut output)?;
        Ok(String::from_utf8(output)?)
    }

    #[test]
    fn does_not_change_normal_yaml() {
        let input = r#"foo: bar
"#;
        let expected = format_string(input).unwrap();
        assert_eq!(input, expected)
    }

    #[test]
    fn unnecessary_whitespace_is_removed() {
        let input = r#"foo:    bar
"#;
        let expected = r#"foo: bar
"#;
        let output = format_string(input).unwrap();
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
        let output = format_string(input).unwrap();
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
        let output = format_string(input).unwrap();
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
        let output = format_string(input).unwrap();
        assert_eq!(expected, output)
    }
}
