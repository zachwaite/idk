```rs
let txt = "...";
let cst = CST::from(txt);
```

```rs
let speclines = txt.split().iter().map(|L| {SpecLine::from(L)});
let specs = speclines.iter().enumerate().fold(|S, (i, L)| { evolve(S, i, L) })
```

```rs
let d_specline:  = DSpecLine::from(txt); // DSpecLine{formtype: Field{value: Idk(), meta: ...}
let d_specline_cont = DSpecLineCont::from(txt);
```
