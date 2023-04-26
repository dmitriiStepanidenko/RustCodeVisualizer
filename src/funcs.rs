use crate::structures::*;
use petgraph::graph::{DiGraph, NodeIndex};
use syn::{File, Item};

pub fn parse_rust_code(code: &str) -> File {
    syn::parse_str(code).unwrap()
}

pub fn filter_structs(parsed: File) -> Vec<Item> {
    parsed
        .items
        .into_iter()
        //filter(|item| matches!(item, Item::Struct(_)))
        .filter(|item| matches!(item, Item::Struct(_) | Item::Enum(_)))
        .collect()
}

pub fn from_item_to_structs(input: Vec<Item>) -> Vec<Element> {
    let mut output = Vec::new();
    for i in input {
        let tmp = Element::try_from(i);
        match tmp {
            Ok(val) => output.push(val),
            Err(_) => {}
        }
    }
    output
}

pub fn create_nodes_for_graph(
    structs: &[Element],
    graph: &mut DiGraph<String, ()>,
) -> Vec<NodeIndex> {
    structs
        .iter()
        .map(|element| match element {
            Element::Struct(struct_element) => graph.add_node(struct_element.name.clone()),
            Element::Enum(enum_element) => graph.add_node(enum_element.name.clone()),
        })
        .collect()
}

fn add_edge_for_simple_type(
    graph: &mut DiGraph<String, ()>,
    node_indices: &[NodeIndex],
    field: &Field,
    field_type: &String,
    element_name: &String,
) {
    if let Some(target_index) = node_indices.iter().position(|&i| graph[i] == *field_type) {
        let source_index = node_indices
            .iter()
            .position(|&i| graph[i] == *element_name)
            .unwrap();
        graph.add_edge(node_indices[source_index], node_indices[target_index], ());
    }
}

fn add_edge_for_vec_type(
    graph: &mut DiGraph<String, ()>,
    node_indices: &[NodeIndex],
    field: &Field,
    field_type: &Type,
    element_name: &String,
) {
    if let Type::Simple(inner_type) = field_type {
        add_edge_for_simple_type(graph, node_indices, field, inner_type, element_name);
    }
    // Add other cases for Other type when needed
}

pub fn add_edges_for_graph(
    elements: &Vec<Element>,
    graph: &mut DiGraph<String, ()>,
    node_indices: &[NodeIndex],
) {
    for element in elements {
        match element {
            Element::Struct(struct_element) => {
                for field in &struct_element.fields {
                    match &field.ty {
                        Type::Simple(field_type) => {
                            add_edge_for_simple_type(
                                graph,
                                node_indices,
                                field,
                                field_type,
                                &struct_element.name,
                            );
                        }
                        Type::Vec(field_type) => {
                            add_edge_for_vec_type(
                                graph,
                                node_indices,
                                field,
                                field_type,
                                &struct_element.name,
                            );
                        }
                        _ => {} // Handle other cases when needed
                    }
                }
            }
            Element::Enum(enum_element) => {
                for variant in &enum_element.variants {
                    for field in &variant.fields {
                        match &field.ty {
                            Type::Simple(field_type) => {
                                add_edge_for_simple_type(
                                    graph,
                                    node_indices,
                                    field,
                                    field_type,
                                    &enum_element.name,
                                );
                            }
                            Type::Vec(field_type) => {
                                add_edge_for_vec_type(
                                    graph,
                                    node_indices,
                                    field,
                                    field_type,
                                    &enum_element.name,
                                );
                            }
                            _ => {} // Handle other cases when needed
                        }
                    }
                }
            }
        }
    }
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
        let items = filter_structs(parsed);
        let elements = from_item_to_structs(items);

        let mut graph = DiGraph::<String, ()>::new();
        let node_indices = create_nodes_for_graph(&elements, &mut graph);

        assert_eq!(graph.node_count(), 7);
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(node_indices.len(), 7);
    }

    #[test]
    fn test_add_edges_for_graph() {
        let parsed = parse_rust_code(TEST_CODE);
        let items = filter_structs(parsed);
        let elements = from_item_to_structs(items);

        let mut graph = DiGraph::<String, ()>::new();
        let node_indices = create_nodes_for_graph(&elements, &mut graph);

        add_edges_for_graph(&elements, &mut graph, &node_indices);

        assert_eq!(graph.node_count(), 7);
        assert_eq!(graph.edge_count(), 6);
    }
    #[test]
    fn test_add_vec_only_edges_for_graph() {
        let code = "
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct C {
            pub f: F,
            pub g: Vec<G>,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct F {}

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct G {}
            ";
        let parsed = parse_rust_code(code);
        let items = filter_structs(parsed);
        let elements = from_item_to_structs(items);

        let mut graph = DiGraph::<String, ()>::new();
        let node_indices = create_nodes_for_graph(&elements, &mut graph);

        add_edges_for_graph(&elements, &mut graph, &node_indices);

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }
}
