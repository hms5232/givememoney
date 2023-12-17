# givememoney

Give me money! A CLI program to help you allocate money.

## Usage

> `gmm` is abbr of "give me money".

```shell
gmm [total to be allocated] [each participant]
```

example:

```shell
gmm 100 40 70
```
result:

```
Total to be allocated: 100
No.1 join allocated event with $40 and should pay $36
No.2 join allocated event with $70 and should pay $64
```

## Install

### Binary

Download from [GitHub Release](https://github.com/hms5232/givememoney/releases) page.

### Source

1. Clone this repo and insure device has cargo installed.
2. `cd` into project dir and
   1. run `cargo run -- [total to be allocated] [each participant]`.
   2. run `cargo build --release` to build binary.

## LICENSE

Copyright © 2023 hms5232

Licensed under [Apache 2.0](https://github.com/hms5232/givememoney/blob/main/LICENSE).
