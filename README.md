# yaml-recfmt

A formatter for YAML files.

YAML can contain multiline strings that are also YAML,
but normal formatters will (understandably) not format
these nested values.

When the `--recursive` flag is provided, this application does so anyway.
Note that this actually changes the data of the string, so use with care.

## Installation

```bash
cargo install --git https://github.com/rudsvar/yaml-recfmt
```

## Examples

Display help.

```bash
yaml-recfmt --help
```

Pipe through `yaml-recfmt`.

```bash
cat input.yaml | yaml-recfmt > output.yaml
```

Format a set of files recursively in-place.

```bash
yaml-recfmt --in-place --recursive examples/*.yaml
```

License: MIT OR Apache-2.0
