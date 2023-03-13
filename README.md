# yaml-recfmt

A formatter for nested YAML files.

YAML can contain multiline strings that are also YAML,
but normal formatters will (understandably) not format
these nested values.

This application does it anyway, which can be useful
for files such as Helm charts with configuration in them.

## Installation

```bash
cargo install --git https://github.com/rudsvar/yaml-recfmt
```

## Examples

Display help.

```bash
yaml-recfmt --help
```

Read from one file and write to another.

```bash
yaml-recfmt input.yaml --output output.yaml
```

Pipe through `yaml-recfmt`.

```bash
cat input.yaml | yaml-recfmt > output.yaml
```

License: MIT OR Apache-2.0
