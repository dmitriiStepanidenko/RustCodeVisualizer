use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use rust_code_visualizer::{
    add_edges_for_graph, create_nodes_for_graph, filter_structs, from_item_to_structs,
    parse_rust_code,
};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_file_path = &args[1];

    let source_code = fs::read_to_string(source_file_path).unwrap();

    let parsed = parse_rust_code(&source_code);
    let structs = from_item_to_structs(filter_structs(parsed));
    let mut graph = DiGraph::<String, ()>::new();
    let node_indices = create_nodes_for_graph(&structs, &mut graph);
    add_edges_for_graph(&structs, &mut graph, &node_indices);

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
