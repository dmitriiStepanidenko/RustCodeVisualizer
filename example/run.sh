#!/bin/sh
cd .. 
cargo run example/structures.rs > example/graph.dot 
cd example
dot -Tpng graph.dot -o graph.png
