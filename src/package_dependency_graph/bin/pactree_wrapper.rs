use std::process::Command;

pub fn get_deps(packages: std::vec::Vec<String>) -> std::vec::Vec<std::vec::Vec<String>> {
    let mut packages_dep_vec = Vec::new();
    for package in packages {
        let mut package_dep_vec: Vec<String> = Vec::new();
        let output = Command::new("pactree")
            .arg("-l")
            .arg("-u")
            .arg(package)
            .output()
            .expect("failed");

        if output.status.success() {
            let string_output = String::from_utf8(output.stdout).unwrap();
            let mut dep_packages: Vec<&str> = string_output.split("\n").collect();
            dep_packages.remove(0);
            for dep_package in dep_packages {
                package_dep_vec.push(dep_package.to_string());
            }
        } else {
            package_dep_vec.push("".to_string());
        }

        println!("dep size: {}", package_dep_vec.len());
        packages_dep_vec.push(package_dep_vec);
    }
    return packages_dep_vec;
}
