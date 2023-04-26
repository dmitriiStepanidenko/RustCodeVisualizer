# Rust code visualizer

A library and CLI tool for graphical representation of Rust code, 
focusing on visualizing the relationships between structs and enums.

[mml](https://crates.io/crates/mml) appears to be dead and deprecated, so I started working on my 
own implementation.


## Usage
Here's how the CLI works (current use):
```sh
cargo run test.rs > graph.dot 
dot -Tpng graph.dot -o graph.png

```

Here's how the CLI works (in future):
```sh
rcvis source.rs source1.rs --format {plantuml,graphviz} --out file.out
```
This command analyzes the provided Rust source files and generates a visual representation 
of the relationships between structs and enums in the specified output format (PlantUML or Graphviz), 
saving the result to file.out.

## Features

- [ ] Export relationships between structs and enums to Graphviz format
- [ ] Export relationships between structs and enums to PlantUML format

## Example
For the structures.rs in example folder:
!(Example structures.rs image)[example/graph.png]

## Installation

To install the Rust Code Visualizer CLI tool, run the following command:
```sh
cargo install rust_code_visualizer
```

Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue on the GitHub repository.
License

Rust Code Visualizer is licensed under the MIT License.
Acknowledgements

Special thanks to the creators of the [mml](https://crates.io/crates/mml) crate for their initial work and inspiration.
