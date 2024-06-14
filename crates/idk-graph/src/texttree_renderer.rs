use crate::cst::{Call, Definition, Program, Statement};

struct TextTreeOutput {
    indent: usize,
    out: String,
}

impl TextTreeOutput {
    fn new() -> Self {
        Self {
            indent: 2,
            out: "MAIN\n".to_string(),
        }
    }

    fn build_call(&mut self, call: &Call) {
        let indent = " ".repeat(self.indent);
        self.out.push_str(&indent);
        self.out.push_str(&call.name);
        self.out.push_str("\n");
    }

    fn build_def(&mut self, def: &Definition) {
        self.indent += 2;
        for call in def.calls.iter() {
            self.build_call(call);
        }
        self.indent -= 2;
    }

    fn build(&mut self, defs: &Vec<Definition>, call: &Call) {
        // main call
        self.build_call(call);
        // any subcalls
        let maybe_found = defs
            .iter()
            .find(|d| d.name.to_uppercase() == call.name.to_uppercase());
        if let Some(def) = maybe_found {
            self.build_def(def);
        }
    }
}

pub fn render_text_tree(pgm: Program) -> String {
    let mut defs: Vec<Definition> = vec![];
    for stmt in pgm.statements.iter() {
        if let Statement::Def(d) = stmt {
            defs.push(d.clone());
        }
    }

    let mut output = TextTreeOutput::new();
    for stmt in pgm.statements.iter() {
        if let Statement::Call(call) = stmt {
            output.build(&defs, call);
        }
    }
    output.out
}
