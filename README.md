# givememoney

Give me money! A CLI program to help you allocate money.

## Usage

> `gmm` is abbr of "give me money".

```shell
gmm [total to be allocated] [money of each participant, can use "=" to specify name]
```

example:

```shell
gmm 100 40 70
```

result:

```
Total to be allocated: $100.00
+----------+----------+-----------+
| No./Name | Original | Allocated |
+----------+----------+-----------+
| 1        |       40 |       $36 |
+----------+----------+-----------+
| 2        |       70 |       $64 |
+----------+----------+-----------+
```

This program also support named participants by using `name=amount` format. Consider the following example:

```
gmm 100 30 bar=20 foo=40 50
```

it will print:

```
Total to be allocated: $100.00
+----------+----------+-----------+
| No./Name | Original | Allocated |
+----------+----------+-----------+
| 1        |       30 |       $21 |
+----------+----------+-----------+
| bar      |       20 |       $14 |
+----------+----------+-----------+
| foo      |       40 |       $29 |
+----------+----------+-----------+
| 4        |       50 |       $36 |
+----------+----------+-----------+
```

## Installation

### Binary

Currently, we provide pre-compiled binaries for the following targets (target triple format):

* x86_64-unknown-linux-gnu
* x86_64-apple-darwin
* x86_64-pc-windows-msvc

Download from [GitHub Release](https://github.com/hms5232/givememoney/releases) page or run
(replace `x86_64-unknown-linux-gnu` with your target):

```shell
wget -O gmm $(curl -s https://api.github.com/repos/hms5232/givememoney/releases/latest | grep "browser_download_url" | grep "x86_64-unknown-linux-gnu" | awk '{ print $2 }' | sed 's/"//g')
```

then `chmod +x gmm` if not executable.

### Source

1. Clone this repo and ensure the device has cargo installed.
2. `cd` into project dir and
    1. run `cargo run -- [total to be allocated] [each participant]`.
    2. run `cargo build --release` to build binary.

## LICENSE

Copyright © 2023 hms5232

Licensed under [Apache 2.0](https://github.com/hms5232/givememoney/blob/main/LICENSE).
