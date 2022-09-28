extern crate petgraph;
mod read_data;
use crate::read_data::read_package_list;
mod pactree_wrapper;
use pactree_wrapper::get_deps;
use petgraph::dot::Dot;
use petgraph::graph::Graph;

fn main() {
    let packages_vec = read_package_list();
    let packages_count = packages_vec.len();
    println!("packages count: {}", packages_count);

    let packages_dep_vec = get_deps(packages_vec);
    let packages_dep_vec_count = packages_dep_vec.len();
    println!("packages dep count: {}", packages_dep_vec_count);
}
