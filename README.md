# IDK

IBM i development tools, for Neovim and the command line.

#### Components:

- `idk.nvim` - a Neovim plugin that works like a language server,
  without the server
- `idk-get` - download source files from an IBM i server via ODBC
- `idk-fmt` - format source files to the proper column width based on
  language
- `rpgle-parser` - a parser for different flavors of RPG. Designed to
  handle programs with `/free` and tradition syntax mixed together without
  mercy. Still a work in progress, but good enough to power "jump to definition"
  on most of the /free and traditional fixed format programs I encounter.
- `dds-parser` - a parser for DDS files. Similar status to the rpg parser.
- `idk-graph` - a source graph generator that outputs dot files to be
  rendered with graphviz.
- `idkpy` - Python bindings to the rpgle-parser. Exposes the `parse_ast` function to
  Python.

## idk-nvim

`idk-nvim` is a Neovim plugin that currently provides syntax highlighting and
basic jump to definition. The core functionality is written in Rust, and
compiled into a shared library for plugin usage via `nvim-oxi`.

###### Jump to definition

Supported jumps:

- local variable usage -> DSpec definition
- subroutine `Exsr` calls -> `Begsr` definition in traditional or `/free` syntax
- field usage -> field definition in a DDS file registered in the project manifest

Not yet supported:
- prefixed field name -> field definition
- external program call -> external program source file

![jumptodefinition](./assets/jumptodefinition.gif)

###### Syntax Highlighting

The RPG highlighter is 2-phase. It first uses the concrete syntax tree to
perform "naive" highlights which have no semantic knowledge of surrounding
fields or other lines. The second phase uses the abstract syntax tree to apply
semantic highlights. An example would be [line
continuations](https://www.ibm.com/docs/en/i/7.4?topic=entries-continuation-rules).
In the screenshot below, the constant string literal value is highlighted in
the second pass, because the d-spec is split across multiple lines.

![syntax](./assets/readme-syntax.png)

###### JSON serialization

Run `:lua require("idk").json_dump_current_buffer("/tmp/dump.json")` to dump the concrete
syntax tree (CST) to a json file. Then use `jq` or a programming language to query the
CST.

###### DOT/Graphviz serialization

The source graph currently visualizes subroutine and external procedure calls as a directed
graph. This is useful for a high level view of control flow when you have to get up to
speed on an existing program.

Run `:lua require("idk").dot_dump_current_buffer("/tmp/dump.gv")` on an open RPG
source file to get a graphviz file. From there run e.g. `dot -Tsvg /tmp/dump.gv > /tmp/dump.svg`
to generate the graph.

![dotrender](./assets/readme-dotrender.svg)


## Download source files from IBM i

Handy for getting source code onto your system versus working with SEU.

```sh
$ DSN=AS400 idk-get "ZWAITE/QRPGLESRC(ZEVT)" | idk-fmt RPG
```

## idkpy

I mostly use this for testing, but can also be used to work with the AST from Python.
Python bindings are created using [maturin](https://github.com/PyO3/maturin)

Example:

```python
import json
import sys
import unittest
from pathlib import Path

import idkpy
from snapshottest import TestCase

WD = Path(__file__).parent
DATA = WD / Path("data")


class TestAST(TestCase):
    def test_parse_test(self):
        fpath = str(DATA / Path("test.rpgle"))
        with open(fpath, "r") as f:
            raw = idkpy.parse_rpgle(f.read())
            parsed = json.loads(raw)
            self.assertMatchSnapshot(parsed)

    def test_parse_hr323(self):
        fpath = str(DATA / Path("HR323.rpgle"))
        with open(fpath, "r") as f:
            raw = idkpy.parse_rpgle(f.read())
            parsed = json.loads(raw)
            self.assertMatchSnapshot(parsed)

    def test_parse_hr111h(self):
        fpath = str(DATA / Path("HR111H.rpgle"))
        with open(fpath, "r") as f:
            raw = idkpy.parse_rpgle(f.read())
            parsed = json.loads(raw)
            self.assertMatchSnapshot(parsed)


def main():
    sys.argv = sys.argv[:1]
    unittest.main(__name__)


if __name__ == "__main__":
    main()
```


## Dependency Graph
```mermaid
flowchart LR
    meta
    field
    cst.srcline
    cst.nvim
    cst
    free
    ast.spec
    ast.nvim
    ast

    meta --> free
    meta --> field
    meta --> cst.srcline
    meta --> cst.nvim
    meta --> ast.nvim
    meta --> ast.spec

    field --> cst.srcline
    field --> ast.spec

    free --> ast.spec

    cst.srcline --> cst.nvim
    cst.nvim --> cst
    cst.srcline --> cst

    cst.srcline --> ast.spec
    cst --> ast.spec

    ast.spec --> ast
    ast.nvim --> ast

    cst --> idk-nvim
    ast --> idk-nvim
```

