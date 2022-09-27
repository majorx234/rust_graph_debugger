extern crate petgraph;
mod read_data;
use crate::read_data::read_package_list;
use petgraph::dot::Dot;
use petgraph::graph::Graph;

fn main() {
    let package_vec = read_data::read_package_list();
    let packages_count = package_vec.len();
    println!("packages count: {}", packages_count);
}
