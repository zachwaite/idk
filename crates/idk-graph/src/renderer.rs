use crate::cst::{Program, Statement};

fn render_node_dot(name: &str) -> String {
    format!(
        r#"
  "{}" [ style = "filled, bold" penwidth = 5 fillcolor = "white" fontname = "Courier New" shape = "Mrecord" label =<
  <table border="0" cellborder="0" cellpadding="3" bgcolor="white">
    <tr>
      <td bgcolor="black" align="center" colspan="2">
        <font color="white">
        {}
        </font>
      </td>
    </tr>
    <tr>
      <td align="left" port="r0">
        -
      </td>
    </tr>
  </table>
  > ];
    "#,
        name, name
    )
}

fn render_edge_dot(parent_name: &str, child_name: &str) -> String {
    format!(
        r#""{}" -> "{}" [ penwidth = 1 fontsize = 14 fontcolor = "grey28" ];"#,
        parent_name, child_name
    )
}

pub fn render_dot(pgm: Program) -> String {
    let mut nodes: Vec<String> = vec![];
    let mut edges: Vec<String> = vec![];

    let snippet = render_node_dot("MAIN");
    nodes.push(snippet);
    for stmt in pgm.statements.iter() {
        if let Statement::Call(call) = stmt {
            let snippet = render_node_dot(&call.name.to_uppercase());
            nodes.push(snippet);
            let snippet = render_edge_dot("MAIN", &call.name.to_uppercase());
            edges.push(snippet);
        }
    }

    for stmt in pgm.statements.iter() {
        if let Statement::Def(def) = stmt {
            for call in def.calls.iter() {
                if !nodes.contains(&call.name.to_uppercase()) {
                    let snippet = render_node_dot(&call.name.to_uppercase());
                    nodes.push(snippet);
                }
                let snippet = render_edge_dot(&def.name.to_uppercase(), &call.name.to_uppercase());
                edges.push(snippet);
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
