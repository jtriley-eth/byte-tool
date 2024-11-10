# Byte "bt" Tool

ever get tired of going to that one website to convert hex to ascii or decimal?

me too, so here's a quick and simple rust cli for exactly that

```bash
Usage: bt [OPTIONS] --input <INPUT> --output <OUTPUT> <DATA>

Arguments:
  <DATA>  

Options:
  -i, --input <INPUT>          input format [possible values: ascii, utf8, hex, dec]
  -o, --output <OUTPUT>        output format [possible values: ascii, utf8, hex, dec]
  -s, --separator <SEPARATOR>  separator string: ', ' = 'xx, xx'
  -g, --grouping <GROUPING>    byte grouping count: 2 = 'xx xx'
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```

example:

```bash
$ bt --input ascii --output hex --grouping 1 --separator ", " asdfasdf
61, 73, 64, 66, 61, 73, 64, 66
```

