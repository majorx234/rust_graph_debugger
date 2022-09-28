use std::collections::HashMap;
extern crate petgraph;
mod read_data;
use crate::read_data::read_package_list;
mod pactree_wrapper;
use pactree_wrapper::get_deps;
use petgraph::dot::Dot;
use petgraph::graph::Graph;

fn dependency_walker(mut to_check: std::vec::Vec<String>) -> HashMap<String, Option<Vec<String>>> {
    let mut package_dep_map: HashMap<String, Option<Vec<String>>> = HashMap::new();
    while let Some(package) = to_check.pop() {
        match package_dep_map.get(&package) {
            Some(_) => (),
            None => {
                let dependencies_opt = get_deps(&package);
                match dependencies_opt {
                    Some(ref dependencies) => {
                        for dep in dependencies {
                            match package_dep_map.get(dep) {
                                Some(_) => (),
                                None => {
                                    to_check.push(dep.clone());
                                }
                            }
                        }
                        package_dep_map.insert(package, dependencies_opt);
                    }
                    None => {
                        package_dep_map.insert(package, None);
                    }
                }
            }
        }
    }
    return package_dep_map;
}

fn create_graph_from_package_dep_list(
    dep_map: HashMap<String, Option<Vec<String>>>,
) -> Graph<String, String> {
    let mut dep_graph: Graph<String, String> = Graph::new();
    for (package, dependencies_opt) in dep_map {
        let package_node = dep_graph.add_node(package);
        match dependencies_opt {
            Some(dependencies) => {
                for dep in dependencies {
                    let dep_node = dep_graph.add_node(dep);
                    dep_graph.add_edge(package_node, dep_node, "".to_string());
                }
            }
            None => (),
        }
    }
    return dep_graph;
}

fn main() {
    let packages_vec = read_package_list();
    let packages_count = packages_vec.len();
    println!("packages count: {}", packages_count);

    let result = dependency_walker(packages_vec);
    let mut dep_graph: Graph<String, String> = create_graph_from_package_dep_list(result);
}
