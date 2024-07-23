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
portpicker 0.1.1
```

# Examples

```text
$ portpicker
21988
```

```text
$ portpicker -n 5
19495
24540
24645
16965
23673
```

