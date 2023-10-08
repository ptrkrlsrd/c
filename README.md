# C
## Unified `cat` & `ls` Functionality in a Single CLI Tool

Unveil the next level of command-line efficiency with Command-C. It's the all-in-one utility that effortlessly merges ls's directory listing and cat's file reading capabilities. Manage files and explore directories with ease, all through a single, powerful command. Note that the tool is not feature aligned with eiter ls nor cat, but I plan to add the most useful flags later.

## Quick Installation

```bash
cargo install c-cli
```

## Features & Usage

* **Directory Listing**: Similar to `ls`, list all files and directories in the current folder.
```bash
c
```
or specify a target folder:
```bash
c {foldername}
```

* **File Content Display**: Like `cat`, read the content of specified files. Supports multiple file arguments.
```bash
c {filename}
```
