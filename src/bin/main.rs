use rust_code_visualizer::{analyze_source_code, generate_dependency_diagram};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_file_path = &args[1];

    let source_code = fs::read_to_string(source_file_path).unwrap();

    let elements = analyze_source_code(&source_code).unwrap();

    let diagram = generate_dependency_diagram

    let diagram = generate_dependency_diagram(elements).unwrap();

    println!("{}", diagram);
}

