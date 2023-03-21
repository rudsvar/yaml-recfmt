use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref QUOTED_MAP: Regex = Regex::new(r#"(?m)^\s*(.*):\s*(['"])(.*)(['"])\s*$"#).unwrap();
    static ref QUOTED_SEQ: Regex = Regex::new(r#"(?m)^\s*-\s*(['"])(.*)(['"])\s*$"#).unwrap();
}

#[derive(Debug)]
struct MapEntry<'a> {
    key: &'a str,
    lquote: &'a str,
    value: &'a str,
    rquote: &'a str,
}

#[derive(Debug)]
struct SequenceElement<'a> {
    lquote: &'a str,
    value: &'a str,
    rquote: &'a str,
}

fn map_entries<'a>(input: &'a str) -> Vec<MapEntry<'a>> {
    let mut kv_pairs = Vec::new();
    for caps in QUOTED_MAP.captures_iter(input) {
        kv_pairs.push(MapEntry {
            key: caps.get(1).unwrap().as_str(),
            lquote: caps.get(2).unwrap().as_str(),
            value: caps.get(3).unwrap().as_str(),
            rquote: caps.get(4).unwrap().as_str(),
        });
    }
    kv_pairs
}

fn sequence_elements<'a>(input: &'a str) -> Vec<SequenceElement<'a>> {
    let mut kv_pairs = Vec::new();
    for caps in QUOTED_SEQ.captures_iter(input) {
        kv_pairs.push(SequenceElement {
            lquote: caps.get(1).unwrap().as_str(),
            value: caps.get(2).unwrap().as_str(),
            rquote: caps.get(3).unwrap().as_str(),
        });
    }
    kv_pairs
}

fn requote_map_entries(original: &str, unquoted: &str) -> String {
    let mut requoted = unquoted.to_string();
    for MapEntry {
        key,
        lquote,
        value,
        rquote,
    } in map_entries(original)
    {
        requoted = requoted.replace(
            &format!("{key}: {value}\n"),
            &format!("{key}: {lquote}{value}{rquote}\n"),
        );
    }
    requoted
}

fn requote_sequence_elements(original: &str, unquoted: &str) -> String {
    let mut requoted = unquoted.to_string();
    for SequenceElement {
        lquote,
        value,
        rquote,
    } in sequence_elements(original)
    {
        requoted = requoted.replace(&format!("- {value}\n"), &format!("- {lquote}{value}{rquote}\n"));
    }
    requoted
}

/// If either a key value pair `key: value` or an element `- value` is
/// found in the original string, and `value` is quoted there, it will
/// also be quoted in the output string.
///
/// Warning: This is currently a bit too aggressive.
/// If the same value is found multiple places, all of them will be quoted.
pub fn requote(original: &str, unquoted: &str) -> String {
    let requoted = requote_map_entries(original, unquoted);
    let requoted = requote_sequence_elements(original, &requoted);
    requoted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quoted_map_key_value_pairs_are_captured() {
        let text = r#"
            a: a .
            b: "b" .
            c: 'c'
        "#;
        let mut caps = QUOTED_MAP.captures_iter(text);

        let caps1 = caps.next().unwrap();
        assert_eq!("b", &caps1[1]);
        assert_eq!("\"", &caps1[2]);
        assert_eq!("b", &caps1[3]);
        assert_eq!("\"", &caps1[4]);

        let caps2 = caps.next().unwrap();
        assert_eq!("c", &caps2[1]);
        assert_eq!("\'", &caps2[2]);
        assert_eq!("c", &caps2[3]);
        assert_eq!("\'", &caps2[4]);
    }

    #[test]
    fn quoted_seq_values_are_captured() {
        let text = r#"
            - a
            - "b"
            - 'c'
        "#;
        let mut caps = QUOTED_SEQ.captures_iter(text);

        let caps1 = caps.next().unwrap();
        assert_eq!("\"", &caps1[1]);
        assert_eq!("b", &caps1[2]);
        assert_eq!("\"", &caps1[3]);

        let caps2 = caps.next().unwrap();
        assert_eq!("\'", &caps2[1]);
        assert_eq!("c", &caps2[2]);
        assert_eq!("\'", &caps2[3]);
    }

    #[test]
    fn requote_requotes_all_values() {
        let original = r#"
            foo: foo
            bar: "bar"
            baz: 'baz'
            - 'test'
        "#;
        let unquoted = r#"
            foo: foo
            bar: bar
            baz: baz
            - test
        "#;
        let requoted = requote(original, unquoted);
        assert_eq!(original, requoted);
    }

    #[ignore = "requoting is still a bit too aggressive"]
    #[test]
    fn requote_doesnt_requote_unquoted() {
        let original = r#"
            foo:
                foo: test
            bar:
                foo: 'test'
        "#;
        let unquoted = r#"
            foo:
                foo: test
            bar:
                foo: test
        "#;
        let requoted = requote(original, unquoted);
        assert_eq!(original, requoted);
    }
}
