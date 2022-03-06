# rulex CLI

This CLI allows you to compile [rulex expressions](../README.md) to regexes in the command line.
It currently requires that [Rust](https://www.rust-lang.org/tools/install) is installed.
Install the CLI with

```sh
cargo install rulex-bin
```

Then you can compile rulex expressions to a regex flavor of your choice; the default is PCRE.

```sh
$ rulex --help
rulex 0.1.0
Ludwig Stecher <ludwig.stecher@gmx.de>
Compile rulex expressions, a new regular expression language

USAGE:
    rulex [OPTIONS] [INPUT]

ARGS:
    <INPUT>    Rulex expression to compile

OPTIONS:
    -d, --debug              Show debug information
    -f, --flavor <FLAVOR>    Regex flavor [possible values: pcre, python,
                             java, javascript, dotnet, ruby, rust]
    -h, --help               Print help information
    -p, --path <FILE>        File containing the rulex expression to compile
    -V, --version            Print version information
```

It provides nice error messages:

```sh
$ rulex "'Hello world'* \X+"
Error:
  × Backslash escapes are not supported
   ╭────
 1 │ 'Hello world'* \X+
   ·                ─┬
   ·                 ╰── error occurred here
   ╰────
  help: Replace `\X` with `X`
```

## License

Dual-licensed under the [MIT license](https://opensource.org/licenses/MIT) or the
[Apache 2.0 license](https://opensource.org/licenses/Apache-2.0).