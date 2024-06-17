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
