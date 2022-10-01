use std::collections::HashMap;
extern crate petgraph;
mod read_data;
use crate::read_data::read_package_list;
mod pactree_wrapper;
use pactree_wrapper::get_deps;
use petgraph::algo;
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

    let mut package_to_graph_node_map = HashMap::new();

    // adding all keys
    for (package, _) in &dep_map {
        package_to_graph_node_map.insert(package.clone(), dep_graph.add_node(package.to_owned()));
    }

    for (package, dependencies_opt) in dep_map {
        let package_node = package_to_graph_node_map.get(&package).unwrap();
        match dependencies_opt {
            Some(dependencies) => {
                for dep in dependencies {
                    let dep_node = package_to_graph_node_map.get(&dep).unwrap();
                    dep_graph.add_edge(*dep_node, *package_node, "".to_string());
                }
            }
            None => (),
        }
    }
    return dep_graph;
}

fn main() {
    let packages_vec = read_package_list();
    let _packages_count = packages_vec.len();

    let result = dependency_walker(packages_vec);

    let dep_graph: Graph<String, String> = create_graph_from_package_dep_list(result);

    let cycles = algo::is_cyclic_directed(&dep_graph);
    if !cycles {
        let toposort_result = algo::toposort(&dep_graph, None);
        match toposort_result {
            Ok(mut topological_ordered_nodes) => {
                topological_ordered_nodes.reverse();
                for node_index in topological_ordered_nodes {
                    match dep_graph.node_weight(node_index) {
                        Some(value) => println!("{}", value),
                        None => (),
                    }
                }
            }
            Err(_) => (),
        }
    }
    // print .dot file
    // println!("{}", Dot::new(&dep_graph));
}
