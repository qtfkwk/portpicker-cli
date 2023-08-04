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
portpicker 0.1.0
```

# Examples

```text
$ portpicker
20451
```

```text
$ portpicker -n 5
18000
24456
23626
18180
23017
```

