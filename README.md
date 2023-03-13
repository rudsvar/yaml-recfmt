# yaml-recfmt

A formatter for nested YAML files.

YAML can contain multiline strings that are also YAML,
but normal formatters will (understandably) not format
these nested values.

This application does it anyway, which can be useful
for files such as Helm charts with configuration in them.

License: MIT OR Apache-2.0
