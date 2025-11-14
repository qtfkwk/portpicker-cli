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
portpicker 0.3.3
```

# Examples

```text
$ portpicker
23051
```

```text
$ portpicker -n 5
24164
20635
18241
18227
19711
```

