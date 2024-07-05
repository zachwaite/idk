use crate::cst::{Call, Definition, Program, Statement};

fn node_color(data: &NodeData) -> String {
    match data.data_type {
        NodeDataType::Main => "black".to_string(),
        NodeDataType::Subroutine => "black".to_string(),
        NodeDataType::ExternalPgm => "slategrey".to_string(),
    }
}

fn render_node_dot(data: &NodeData) -> String {
    let mut mutsnips = "".to_string();
    for snip in data.mutations.iter() {
        let x = format!(r#"<tr><td align="left" port="r0">{}</td></tr>"#, snip);
        mutsnips.push_str(&x);
    }
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
    {}
  </table>
  > ];
    "#,
        data.name,
        node_color(data),
        data.name,
        mutsnips
    )
}

fn render_edge_dot(parent_name: &str, child_name: &str) -> String {
    format!(
        r#""{}" -> "{}" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];"#,
        parent_name, child_name
    )
}

enum NodeDataType {
    Main,
    Subroutine,
    ExternalPgm,
}

struct NodeData {
    data_type: NodeDataType,
    name: String,
    mutations: Vec<String>,
}

pub fn render_dot(pgm: Program) -> String {
    let mut nodes: Vec<String> = vec![];
    let mut edges: Vec<String> = vec![];
    let mut mutations: Vec<(String, Vec<String>)> = vec![];

    // gather muts from defs
    for stmt in pgm.statements.iter() {
        if let Statement::Def(Definition::Subroutine(def)) = stmt {
            let mut muts = def
                .mutations
                .iter()
                .map(|x| format!("{}: {}", x.keyword, x.name))
                .collect::<Vec<String>>();
            muts.sort();
            muts.dedup();
            let name = def.name.to_uppercase();
            mutations.push((name, muts));
        }
    }

    // add main node
    let mut main_muts: Vec<String> = vec![];
    for stmt in pgm.statements.iter() {
        if let Statement::Mutation(m) = stmt {
            let m = format!("{}: {}", m.keyword, m.name);
            if !main_muts.contains(&m) {
                main_muts.push(m);
            }
        }
    }
    let data = NodeData {
        data_type: NodeDataType::Main,
        name: "MAIN".to_string(),
        mutations: main_muts,
    };
    let snippet = render_node_dot(&data);
    nodes.push(snippet);

    // add nodes for top level calls
    for stmt in pgm.statements.iter() {
        if let Statement::Call(Call::Subroutine(call)) = stmt {
            let maybe = mutations
                .iter()
                .find(|x| x.0.to_string() == call.name.to_uppercase());
            let muts = match maybe {
                Some(m) => m.1.clone(),
                None => vec![],
            };
            let data = NodeData {
                data_type: NodeDataType::Subroutine,
                name: call.name.to_uppercase(),
                mutations: muts,
            };
            let snippet = render_node_dot(&data);
            nodes.push(snippet);
            let snippet = render_edge_dot("MAIN", &call.name.to_uppercase());
            if !edges.contains(&snippet) {
                edges.push(snippet);
            }
        }
        if let Statement::Call(Call::ExternalPgm(call)) = stmt {
            let data = NodeData {
                data_type: NodeDataType::ExternalPgm,
                name: call.name.to_uppercase(),
                mutations: vec![],
            };
            let snippet = render_node_dot(&data);
            nodes.push(snippet);
            let snippet = render_edge_dot("MAIN", &call.name.to_uppercase());
            if !edges.contains(&snippet) {
                edges.push(snippet);
            }
        }
    }

    // add nodes for calls inside other calls
    for stmt in pgm.statements.iter() {
        if let Statement::Def(Definition::Subroutine(def)) = stmt {
            for call in def.calls.iter() {
                match call {
                    Call::Subroutine(c) => {
                        if !nodes.contains(&c.name.to_uppercase()) {
                            let maybe = mutations
                                .iter()
                                .find(|x| x.0.to_string() == c.name.to_uppercase());
                            let muts = match maybe {
                                Some(m) => m.1.clone(),
                                None => vec![],
                            };
                            let data = NodeData {
                                data_type: NodeDataType::Subroutine,
                                name: c.name.to_uppercase(),
                                mutations: muts,
                            };
                            let snippet = render_node_dot(&data);
                            nodes.push(snippet);
                        }
                        let snippet =
                            render_edge_dot(&def.name.to_uppercase(), &c.name.to_uppercase());
                        if !edges.contains(&snippet) {
                            edges.push(snippet);
                        }
                    }
                    Call::ExternalPgm(c) => {
                        if !nodes.contains(&c.name.to_uppercase()) {
                            let data = NodeData {
                                data_type: NodeDataType::ExternalPgm,
                                name: c.name.to_uppercase(),
                                mutations: vec![],
                            };
                            let snippet = render_node_dot(&data);
                            nodes.push(snippet);
                        }
                        let snippet =
                            render_edge_dot(&def.name.to_uppercase(), &c.name.to_uppercase());
                        if !edges.contains(&snippet) {
                            edges.push(snippet);
                        }
                    }
                }
            }
        }
    }
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
        nodes.join("\n"),
        edges.join("\n")
    )
}
