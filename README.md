# RPP (Rust Pre-Processor)

RPP is a text pre-processor written in Rust.
This project is largely inspired by [PP, the POSIX shell text  preprocessor](https://github.com/jhjn/pp).

As such, their usage and syntax are very similar.

## Usage

Typical rpp usage is very simple.

```
rpp <filename>
```

Any valid shell command can be placed after two exclamation marks (!!) in any
line and RPP will run the command and replace the line with the commands output.

```
# Sample Markdown

Numbers:
!! for i in $(seq 1 10); do echo "$(($i * 2))"; done
```

This will expand to 
```
# Sample Markdown

Numbers:
2
4
6
8
10
12
14
16
18
20
```

## Configuration

RPP also support one other syntax. The !# works similarly to a hashbang in shell
scripts and will change the processor to some other program.

```
!# sh - changes shell to sh
!# bash - changes shell to bash 
```

Sh is the default program. The !# syntax could theoretically run any program
that uses the format `<program name> -c <command>`.

## Building

To compile this, you'll need to have the Rust toolchain installed.
After that it's as simple as cloning the repo and running `cargo build`.

## Contribution

This started out as a project by students who were just learning Rust. As such,
there will likely be bugs and non-idiomatic ways of doing things here.

Feel free to submit a Pull Request if you wish to help.

## License

[MIT](./LICENSE)
