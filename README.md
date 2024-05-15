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

result (since v0.2.0):

```
Total to be allocated: 100
+-----+----------+-----------+
| No. | Original | Allocated |
+-----+----------+-----------+
| 1   |       40 |        36 |
+-----+----------+-----------+
| 2   |       70 |        64 |
+-----+----------+-----------+
```

result (only v0.1.0):

```
Total to be allocated: 100
No.1 join allocated event with $40 and should pay $36
No.2 join allocated event with $70 and should pay $64
```

------

since v0.2.1, support `name=amount` format. Consider the following example:

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

## Install

### Binary

Download from [GitHub Release](https://github.com/hms5232/givememoney/releases) page (and `chmod +x gmm` if not executable).

### Source

1. Clone this repo and insure device has cargo installed.
2. `cd` into project dir and
   1. run `cargo run -- [total to be allocated] [each participant]`.
   2. run `cargo build --release` to build binary.

## LICENSE

Copyright © 2023 hms5232

Licensed under [Apache 2.0](https://github.com/hms5232/givememoney/blob/main/LICENSE).
