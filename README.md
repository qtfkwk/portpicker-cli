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
portpicker 0.3.0
```

# Examples

```text
$ portpicker
23557
```

```text
$ portpicker -n 5
21648
16295
16929
19070
23835
```

