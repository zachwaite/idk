use rpgle_parser;
use rpgle_parser::{FieldResult, Op, Spec, AST};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

// rendering
pub trait IRenderable {
    fn render(&self) -> String;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub left: String,
    pub right: String,
}
impl IRenderable for Edge {
    fn render(&self) -> String {
        format!(
            r#""{}" -> "{}" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];"#,
            self.left, self.right
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    Main,
    Subroutine,
    Extpgm,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    name: String,
    kind: NodeKind,
}
impl IRenderable for Node {
    fn render(&self) -> String {
        let node_color = match self.kind {
            NodeKind::Main => "black".to_string(),
            NodeKind::Subroutine => "black".to_string(),
            NodeKind::Extpgm => "slategrey".to_string(),
        };
        format!(
            r#"
  "{}" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="{}" align="center" colspan="2">
        <font color="white">
        {}
        </font>
      </td>
    </tr>
  </table>
  > ];
    "#,
            self.name, node_color, self.name
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Component {
    Node(Node),
    Edge(Edge),
}
impl Component {
    pub fn is_node(&self) -> bool {
        match self {
            Self::Node(_) => true,
            Self::Edge(_) => false,
        }
    }
    pub fn is_edge(&self) -> bool {
        match self {
            Self::Node(_) => false,
            Self::Edge(_) => true,
        }
    }
}
impl IRenderable for Component {
    fn render(&self) -> String {
        match self {
            Component::Node(n) => n.render(),
            Component::Edge(e) => e.render(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdkGraph {
    pub components: Vec<Component>,
}
impl IRenderable for IdkGraph {
    fn render(&self) -> String {
        format!(
            r#"
digraph g {{
  fontname="Helvetica,Arial,sans-serif"
  node [fontname="Helvetica,Arial,sans-serif"]
  edge [fontname="Helvetica,Arial,sans-serif"]
  graph [fontsize=30 labelloc="t" label="" splines=true overlap=false rankdir = "LR"];
  ratio = auto;
  {}
  {}
}}
"#,
            self.components
                .iter()
                .filter(|x| x.is_node())
                .map(|x| x.render())
                .collect::<Vec<String>>()
                .join("\n"),
            self.components
                .iter()
                .filter(|x| x.is_edge())
                .map(|x| x.render())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
impl From<Model> for IdkGraph {
    fn from(value: Model) -> Self {
        let _ = std::fs::write("/tmp/model.txt", format!("{:#?}", &value));
        let mut graph = Self { components: vec![] };
        // root node
        graph.components.push(Component::Node(Node {
            name: "MAIN".to_string(),
            kind: NodeKind::Main,
        }));
        for stmt in value.statements.iter() {
            // defs = nodes
            if let Statement::Def(defstmt) = stmt {
                if let Definition::ExtPgm(def) = defstmt {
                    let node = Component::Node(Node {
                        name: def.to_uppercase().to_string(),
                        kind: NodeKind::Extpgm,
                    });
                    if !graph.components.contains(&node) {
                        graph.components.push(node);
                    }
                }
                if let Definition::Subroutine(def) = defstmt {
                    let node = Component::Node(Node {
                        name: def.name.to_uppercase().to_string(),
                        kind: NodeKind::Subroutine,
                    });
                    if !graph.components.contains(&node) {
                        graph.components.push(node);
                    }
                }
            }
            // calls = edges
            if let Statement::Call(callstmt) = stmt {
                let edge = Component::Edge(Edge {
                    left: "MAIN".to_string(),
                    right: callstmt.name(),
                });
                if !graph.components.contains(&edge) {
                    graph.components.push(edge);
                }
            }
        }
        // calls inside defs = more edges
        for stmt in value.statements.iter() {
            if let Statement::Def(defstmt) = stmt {
                if let Definition::Subroutine(def) = defstmt {
                    for call in def.calls.iter() {
                        let edge = Component::Edge(Edge {
                            left: def.name.to_uppercase(),
                            right: call.name(),
                        });
                        if !graph.components.contains(&edge) {
                            graph.components.push(edge);
                        }
                    }
                }
            }
        }
        graph
    }
}
impl From<&AST> for IdkGraph {
    fn from(value: &AST) -> Self {
        let state = ParserState { idx: 0 };
        let parser = Parser {
            state: RefCell::new(state),
            input: value.specs.iter().map(|x| x.clone()).collect::<Vec<Spec>>(),
        };
        let mut statements = vec![];
        loop {
            match next_statement(&parser) {
                Ok(stmt) => statements.push(stmt),
                Err(x) => {
                    if x == "END".to_string() {
                        break;
                    }
                }
            }
        }
        let model = Model { statements };
        Self::from(model)
    }
}

// model

#[derive(Debug)]
pub enum Call {
    Subroutine(String),
    ExtPgm(String),
}
impl Call {
    pub fn name(&self) -> String {
        match self {
            Call::Subroutine(call) => call.to_string(),
            Call::ExtPgm(call) => call.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SubroutineDefinition {
    pub name: String,
    pub calls: Vec<Call>,
}

#[derive(Debug)]
pub enum Definition {
    Subroutine(SubroutineDefinition),
    ExtPgm(String),
}

#[derive(Debug)]
pub enum Statement {
    Call(Call),
    Def(Definition),
}

#[derive(Debug)]
pub struct Model {
    pub statements: Vec<Statement>,
}

// parsing
#[derive(Debug)]
struct ParserState {
    idx: usize,
}

#[derive(Debug)]
struct Parser {
    state: RefCell<ParserState>,
    input: Vec<Spec>,
}

fn peek_n(parser: &Parser, n: usize) -> Option<&Spec> {
    let idx = parser.state.borrow().idx;
    parser.input.get(idx + n)
}

fn read_spec(parser: &Parser) -> &Spec {
    let out = peek_n(parser, 0).expect("read_line() requires a length check prior to call");
    parser.state.borrow_mut().idx += 1;
    out
}

fn next_statement(parser: &Parser) -> Result<Statement, String> {
    let Some(_) = peek_n(parser, 0) else {
        return Err("END".to_string());
    };
    let pass = "PASS".to_string();
    match read_spec(parser) {
        Spec::D {
            name,
            definition_type,
            ..
        } => {
            if let FieldResult::Ok(dtfield) = definition_type {
                if dtfield.value.is_pr() {
                    if let FieldResult::Ok(namefield) = name {
                        let name = namefield.value.to_string();
                        let _ = read_spec(parser);
                        let def = Definition::ExtPgm(name);
                        return Ok(Statement::Def(def));
                    }
                }
            }
            Err(pass)
        }
        Spec::C { code } => {
            if let FieldResult::Ok(codefield) = code {
                // defs
                if let Op::Begsr { name, .. } = &codefield.op {
                    // collect any calls inside the def
                    let mut calls = vec![];
                    loop {
                        if let Some(_) = peek_n(parser, 0) {
                            let spec = read_spec(parser);
                            if let Spec::C {
                                code: FieldResult::Ok(codefield),
                            } = spec
                            {
                                if let Op::Endsr { .. } = &codefield.op {
                                    break;
                                }
                                if let Op::Exsr { name, .. } = &codefield.op {
                                    let call = Call::Subroutine(name.to_uppercase().clone());
                                    calls.push(call);
                                    continue;
                                }
                                if let Op::Callp { name, .. } = &codefield.op {
                                    let call = Call::ExtPgm(name.to_uppercase().clone());
                                    calls.push(call);
                                    continue;
                                }
                            }
                            continue;
                        }
                        break;
                    }
                    let def = Definition::Subroutine(SubroutineDefinition {
                        name: name.to_uppercase().to_string(),
                        calls,
                    });
                    return Ok(Statement::Def(def));
                }
                // calls
                if let Op::Exsr { name, .. } = &codefield.op {
                    let call = Call::Subroutine(name.to_uppercase().to_string());
                    return Ok(Statement::Call(call));
                }
                if let Op::Callp { name, .. } = &codefield.op {
                    let call = Call::ExtPgm(name.to_uppercase().to_string());
                    return Ok(Statement::Call(call));
                }
            }
            Err(pass)
        }
        Spec::H { .. } => Err(pass),
        Spec::F { .. } => Err(pass),
    }
}
