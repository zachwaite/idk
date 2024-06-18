# IDK - Language Tooling for IBM i

## Neovim Syntax Highlighter

![screenshot-syntax](./screenshots/readme-syntax.png)

## CLI Source Code Visualization

### text-tree renderer

```
$ idk-graph text-tree demo.rpgle
MAIN
  $SetLstId
  $CrtEvts
    $CrtCowEvt
    $CrtBrnEvt
```

### dot renderer

input:
![dotrender](./screenshots/readme-dotrender-src.png)

Run idk-graph:

```bash
idk-graph dot ./demo.rpgle > ./graph.gv
dot -Tsvg ./graph.gv > ./graph.svg
```

Output:

![dotrender](./screenshots/readme-dotrender.svg)

## Download source files from IBM i

```sh
$ DSN=AS400 idk-get "ZWAITE/QRPGLESRC(ZEVT)" | idk-fmt RPG
```

## Set marks at each spectype

- `'h` = jump to mark at beginning of h-specs
- `'f` = jump to mark at beginning of f-specs
- `'d` = jump to mark at beginning of d-specs
- `'i` = jump to mark at beginning of i-specs
- `'c` = jump to mark at beginning of c-specs
- `'o` = jump to mark at beginning of o-specs
