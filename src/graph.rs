use std::collections::HashMap;
use std::rc::Rc;

use crate::Value;
use dot_structures::{Attribute, NodeId, Vertex};
use graphviz_rust::{dot_structures, exec};

use graphviz_rust::cmd::Format;
use graphviz_rust::dot_structures::{Edge, EdgeTy, Graph, Id, Node, Stmt};
use graphviz_rust::printer::PrinterContext;

fn build_dot(nodes: &Vec<Rc<Value>>, edges: &Vec<(Rc<Value>, Rc<Value>)>) -> Graph {
    let mut stmts = vec![];
    let mut node_ids = HashMap::new();

    // Assign each node a unique ID based on its index
    for (i, node) in nodes.iter().enumerate() {
        node_ids.insert(node.clone(), i); // assuming Value: Eq + Hash
        let lbl = format!(
            "\"val {:.2}\\lgrad: {:.2}{}\"",
            node.number,
            node.gradient,
            node.operation
                .map(|op| format!("\\lop: {}", op))
                .unwrap_or_default()
        );

        stmts.push(Stmt::Node(Node {
            id: NodeId(Id::Plain(i.to_string()), None),
            attributes: vec![
                Attribute(Id::Plain("label".to_string()), Id::Plain(lbl.to_string())),
                Attribute(
                    Id::Plain("shape".to_string()),
                    Id::Plain("record".to_string()),
                ),
            ],
        }));
    }

    for (src, dst) in edges {
        let src_id = node_ids.get(src).unwrap();
        let dst_id = node_ids.get(dst).unwrap();

        stmts.push(Stmt::Edge(Edge {
            ty: EdgeTy::Pair(
                Vertex::N(NodeId(Id::Plain(src_id.to_string()), None)),
                Vertex::N(NodeId(Id::Plain(dst_id.to_string()), None)),
            ),
            attributes: vec![],
        }));
    }

    Graph::Graph {
        strict: false,
        id: Id::Plain("G".to_string()),
        stmts,
    }
}

fn build_graph(
    value: Rc<Value>,
    nodes: &mut Vec<Rc<Value>>,
    edges: &mut Vec<(Rc<Value>, Rc<Value>)>,
) {
    if !nodes.contains(&value) {
        nodes.push(value.clone());
    }

    for child in &value.children {
        if !nodes.contains(child) {
            nodes.push(child.clone());
        }

        edges.push((child.clone(), value.clone()));
        build_graph(child.clone(), nodes, edges);
    }
}

fn store_graph(g: Graph) {
    let graph_svg = exec(g, &mut PrinterContext::default(), vec![Format::Svg.into()]).unwrap();

    let mut file = std::fs::File::create("graph.svg").unwrap();
    std::io::Write::write_all(&mut file, graph_svg.as_slice()).unwrap();
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use super::*;

    #[test]
    fn test_display_graph() {
        let value = Value::new(5.0) * Value::new(10.0) + Value::new(3.0) / Value::new(8.0);
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        build_graph(value.into(), &mut nodes, &mut edges);

        let dot_graph = build_dot(&nodes, &edges);
        println!("{:?}", dot_graph);

        let svg = exec(
            dot_graph,
            &mut PrinterContext::default(),
            vec![Format::Svg.into()],
        )
        .expect("Graphviz command failed");
        let mut file = File::create("graph.svg").expect("Unable to create file");
        file.write_all(svg.as_slice())
            .expect("Unable to write SVG content");
    }

    #[test]
    fn test_build_graph() {
        let value = Value::new(5.0) * Value::new(10.0) + Value::new(3.0) / Value::new(8.0);

        println!("{:?}\n\n", value);
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        build_graph(Rc::new(value), &mut nodes, &mut edges);
        println!("Here is the graph that was built:\n");
        println!("Nodes:\n{:?}", nodes);
        println!("Edges:\n{:?}", edges);
    }
}
