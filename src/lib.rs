//! A formatter for nested YAML files.
//!
//! YAML can contain multiline strings that are also YAML,
//! but normal formatters will (understandably) not format
//! these nested values.
//!
//! This application does it anyway, which can be useful
//! for files such as Helm charts with configuration in them.

use std::io::{Read, Write};

use serde_yaml::Value;

/// Parses and formats nested strings found in the YAML value.
pub fn format(value: Value) -> Value {
    match value {
        Value::String(s) => match serde_yaml::from_str::<Value>(&s) {
            // Inner value was just a string, keep it.
            Ok(s @ Value::String(_)) => s,
            // Inner value is more yaml, recurse
            Ok(v) => {
                let formatted = serde_yaml::to_string(&format(v));
                Value::String(formatted.expect("failed to serialize yaml"))
            }
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
pub fn run_format<R: Read, W: Write>(input: R, output: W) -> color_eyre::Result<()> {
    // Run formatter on input
    let yaml: Value = serde_yaml::from_reader(input)?;
    let formatted = format(yaml);

    // Write formatted value to output
    serde_yaml::to_writer(output, &formatted)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn format_string(yaml: &str) -> color_eyre::Result<String> {
        let input = Cursor::new(yaml);
        let mut output = Vec::new();
        run_format(input, &mut output)?;
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
}
