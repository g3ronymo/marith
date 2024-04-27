# marith 

Train mental arithmetic.

*mentalArithmetic* serves a html page on http-get requests.
By default mentalArithmetic will try to bind to 127.0.0.1:8014 

## Usage

```
Usage: marith [OPTIONS] <TEMPLATE_PATH>

Arguments:
  <TEMPLATE_PATH>  Path to template file

Options:
  -p <PORT>      Port number to use [default: 8014]
  -6             Use ipv6 instead of the default ipv4
  -a             Listen to all available addresses (by default listen only to localhost)
  -h, --help     Print help
```

TEMPLATE_PATH is the path to the html template (template/template.html).
