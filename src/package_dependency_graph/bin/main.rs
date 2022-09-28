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

fn main() {
    let packages_vec = read_package_list();
    let packages_count = packages_vec.len();
    println!("packages count: {}", packages_count);

    let result = dependency_walker(packages_vec);
    println!("{:?}", result);
}
