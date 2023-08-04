# C
## cat and ls combined into one command

This CLI program emulates the basic functionality of the `ls` command found in Unix-like systems combined with `cat`. It lists the names of all files and directories in the current directory as well as showing the file content of the specified file.

## Installing

```bash
cargo install c-cli
```

## Usage

* List files in current directory, just like `ls`
```
c
```

or

```
c {foldername}
```

* Show file content like `cat`. You can also specify multiple files
```
c {filename}
```
