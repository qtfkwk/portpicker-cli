# About

Pick a free unused port (CLI)

# Usage

```text
$ portpicker -h
Pick a free unused port (CLI)

Usage: portpicker [OPTIONS]

Options:
  -n <N>         Pick N free unused ports [default: 1]
  -h, --help     Print help
  -V, --version  Print version
```

```text
$ portpicker -V
portpicker 0.2.3
```

# Examples

```text
$ portpicker
19620
```

```text
$ portpicker -n 5
16491
16479
18265
17661
21163
```

