# IDK

IBM i development tools, for Neovim and the command line.

#### Components:

- `idk.nvim` (WIP) - the Neovim plugin that works like a language server, without the server
- `idk-get` (READY) - download source files from an IBM i server via ODBC
- `idk-fmt` (READY) - format source files to the proper column width based on language
- `rpgle-parser` (WIP) - a parser for different flavors of RPG
- `dds-parser` (WIP) - a parser for DDS files
- `idk-graph` (WIP) - a source graph generator that outputs dot files to be rendered with graphviz
- `idk-lint` (TODO) - someday there will be a linter

## idk-nvim

`idk-nvim` is a Neovim plugin that currently provides syntax highlighting and
basic jump to definition. The core functionality is written in Rust, and
compiled into a shared library for plugin usage via `nvim-oxi`.

###### Jump to definition

![jumptodefinition](./assets/jumptodefinition.gif)

###### Syntax Highlighting

![screenshot-syntax](./assets/readme-syntax.png)


## Download source files from IBM i

```sh
$ DSN=AS400 idk-get "ZWAITE/QRPGLESRC(ZEVT)" | idk-fmt RPG
```

