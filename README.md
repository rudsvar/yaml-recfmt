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
yaml-recfmt -h
```

Pipe YAML from one file to another.

```bash
cat input.yaml | yaml-recfmt > output.yaml
```

Read from one file and write to another.

```bash
yaml-recfmt --input input.yaml --output output.yaml
```

Format a file in-place.

```bash
yaml-recfmt input.yaml
```

License: MIT OR Apache-2.0
