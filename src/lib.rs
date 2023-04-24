extern crate quote;
extern crate syn;

use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub fn analyze_source_code(source_code: &str) -> Result<Vec<Element>, MmlError> {
    todo!()
}

pub fn generate_dependency_diagram(elements: Vec<Element>) -> Result<String, MmlError> {
    todo!()
}

use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use std::fs;
use syn::{File, Item, ItemStruct, Type};

fn parse_rust_code(code: &str) -> File {
    syn::parse_str(code).unwrap()
}

fn filter_structs(parsed: File) -> Vec<Item> {
    parsed
        .items
        .into_iter()
        .filter(|item| matches!(item, Item::Struct(_)))
        .collect()
}

fn create_nodes_for_graph(structs: &[Item], graph: &mut DiGraph<String, ()>) -> Vec<NodeIndex> {
    structs
        .iter()
        .map(|item| match item {
            Item::Struct(item_struct) => graph.add_node(item_struct.ident.to_string()),
            _ => unreachable!(),
        })
        .collect()
}

fn add_edges_for_graph(
    structs: &Vec<Item>,
    graph: &mut DiGraph<String, ()>,
    node_indices: &[NodeIndex],
) {
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CODE: &str = r#"
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct A {
            pub b: B,
            pub c: C,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct B {
            pub d: D,
            pub e: E,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct C {
            pub f: F,
            pub g: Vec<G>,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct D {}

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct E {}

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct F {}

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct G {}
    "#;

    #[test]
    fn test_parse_rust_code() {
        let parsed = parse_rust_code(TEST_CODE);
        assert_eq!(parsed.items.len(), 7);
    }

    #[test]
    fn test_filter_structs() {
        let parsed = parse_rust_code(TEST_CODE);
        let structs = filter_structs(parsed);
        assert_eq!(structs.len(), 7);
    }

    #[test]
    fn test_create_nodes_for_graph() {
        let parsed = parse_rust_code(TEST_CODE);
        let structs = filter_structs(parsed);

        let mut graph = DiGraph::<String, ()>::new();
        let node_indices = create_nodes_for_graph(&structs, &mut graph);

        assert_eq!(graph.node_count(), 7);
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(node_indices.len(), 7);
    }

    #[test]
    fn test_add_edges_for_graph() {
        let parsed = parse_rust_code(TEST_CODE);
        let structs = filter_structs(parsed);

        let mut graph = DiGraph::<String, ()>::new();
        let node_indices = create_nodes_for_graph(&structs, &mut graph);
        println!("node_indices:{:?}", node_indices);

        add_edges_for_graph(&structs, &mut graph, &node_indices);
        println!("\ngraph:{:?}", graph);

        assert_eq!(graph.node_count(), 7);
        assert_eq!(graph.edge_count(), 6);
    }
}




