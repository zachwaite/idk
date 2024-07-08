# AST queries to implement

- what files are read/written?
- what fields from each file are read/written?
- what global state is required for input?
- 

# Utils to build

## idk-graph

- Generate graph of the procedure/subprocedure/subroutine call stack
- Generate graph of the mutation of a particular variable
- 

## plugins

- syntax highlighter
- formatter
- linter
- refactoring tool

- dspf
- pfdds
- lfdds
- rpgle
- clle

# Analysis Workflow

1. subroutine graphs
2. identify all effects in the program
3. 

# CST

```rust
enum CSpec {
    Free(FreeCspec),
    ExtF2(ExtF2CSpec),
    Traditional(TraditionalCSpec),
}

enum Spec {
    Control(HSpec),
    Calculation(CSpec),
    Comment,
    CompilerDirective,
    ...
}


enum Program {
    FullFree(Vec<FullFreeSpec>),
    NotFullFree(Vec<Spec>),
}

# 3 phase parser
1. classification (txt -> Vec<RawSpec>) - classify rows as a spec type and handle line continuation (parallel)
2. tokenization - (Vec<RawSpec> -> Vec<Spec>) convert each RawSpec into a Spec instance holding a struct of tokens for each slot (parallel)
3. parsing - (Vec<Spec> -> Vec<Statement>) convert Specs into semantically aware statements
```
