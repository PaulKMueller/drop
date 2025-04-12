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
    let mut node_ids: HashMap<*const Value, usize> = HashMap::new();

    stmts.push(Stmt::Attribute(Attribute(
        Id::Plain("rankdir".to_string()),
        Id::Plain("LR".to_string()),
    )));

    for (i, node) in nodes.iter().enumerate() {
        if let Some(op) = node.operation {
            let op_node_id = format!("op_{}", i);

            // Add the op node
            stmts.push(Stmt::Node(Node {
                id: NodeId(Id::Plain(op_node_id.clone()), None),
                attributes: vec![
                    Attribute(
                        Id::Plain("label".to_string()),
                        Id::Plain(format!("\"{}\"", op)),
                    ),
                    Attribute(
                        Id::Plain("shape".to_string()),
                        Id::Plain("circle".to_string()),
                    ),
                ],
            }));

            let edge_stmt = Stmt::Edge(Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(Id::Plain(op_node_id), None)),
                    Vertex::N(NodeId(Id::Plain(i.to_string()), None)),
                ),
                attributes: vec![],
            });
            stmts.push(edge_stmt);
        }
        node_ids.insert(Rc::as_ptr(node), i);
        let lbl = format!(
            "\"val {:.4}\\lgrad: {:.4}{}\"",
            node.number,
            node.gradient.take(),
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
        for (src, dst) in edges {
            if !node_ids.contains_key(&Rc::as_ptr(src)) {
                eprintln!("Missing src: {:?}", Rc::as_ptr(src));
            }
            if !node_ids.contains_key(&Rc::as_ptr(dst)) {
                eprintln!("Missing dst: {:?}", Rc::as_ptr(dst));
            }
        }
        let src_id = node_ids.get(&Rc::as_ptr(src)).unwrap();
        let dst_id = node_ids.get(&Rc::as_ptr(dst)).unwrap();

        if dst.operation.is_some() {
            let op_node_id = format!("op_{}", dst_id);
            stmts.push(Stmt::Edge(Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(Id::Plain(src_id.to_string()), None)),
                    Vertex::N(NodeId(Id::Plain(op_node_id), None)),
                ),
                attributes: vec![],
            }));
        } else {
            stmts.push(Stmt::Edge(Edge {
                ty: EdgeTy::Pair(
                    Vertex::N(NodeId(Id::Plain(src_id.to_string()), None)),
                    Vertex::N(NodeId(Id::Plain(dst_id.to_string()), None)),
                ),
                attributes: vec![],
            }));
        }
    }

    Graph::DiGraph {
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
    if !nodes.iter().any(|n| Rc::ptr_eq(n, &value)) {
        nodes.push(value.clone());
    }

    for child in &value.children.take() {
        if !nodes.iter().any(|n| Rc::ptr_eq(n, child)) {
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

    use crate::value::V;

    use super::*;

    #[test]
    fn test_display_graph() {
        let a = V::new(5.0);
        let b = V::new(5.0);
        let c = V::new(10.0);
        let d = V::new(3.0);
        let e = V::new(8.0);
        let f = V::new(10.0);
        
        let ab = &a + &b;
        let ef = &e + &f;
        let left = &ab * &c;
        let right = &d / &ef;
        let result = &left + &right;
        
        result.backpropagate();
        
        println!("a ptr = {:p}", &*a.0);
        println!("b ptr = {:p}", &*b.0);
        println!("c ptr = {:p}", &*c.0);
        println!("d ptr = {:p}", &*d.0);
        println!("e ptr = {:p}", &*e.0);
        println!("f ptr = {:p}", &*f.0);
        
        println!("∂L/∂a = {}", a.grad());
        println!("∂L/∂b = {}", b.grad());
        println!("∂L/∂c = {}", c.grad());
        println!("∂L/∂d = {}", d.grad());
        println!("∂L/∂e = {}", e.grad());
        println!("∂L/∂f = {}", f.grad());

        println!("a = {:p}", &*a.0);
        println!("b = {:p}", &*b.0);

        for child in result.0.children.borrow().iter() {
            println!("child: {:?}", child.number);
            for grand in child.children.borrow().iter() {
                println!("  grandchild: {:?}", grand.number);
            }
        }

        // let mut value = Value::new(11.0) / Value::new(23.00);
        // println!("{:?}", value.gradient);

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        build_graph(result.0.clone(), &mut nodes, &mut edges);

        let dot_graph = build_dot(&nodes, &edges);
        // println!("{:?}", dot_graph);

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

    // #[test]
    // fn test_build_graph() {
    //     let value = (Value::new(5.0) + Value::new(5.0)) * Value::new(10.0)
    //         + Value::new(3.0) / (Value::new(8.0) + Value::new(10.0));

    //     println!("{:?}\n\n", value);
    //     let mut nodes = Vec::new();
    //     let mut edges = Vec::new();
    //     build_graph(Rc::new(value), &mut nodes, &mut edges);
    //     println!("Here is the graph that was built:\n");
    //     println!("Nodes:\n{:?}", nodes);
    //     println!("Edges:\n{:?}", edges);
    // }
}
