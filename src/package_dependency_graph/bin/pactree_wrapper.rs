use std::process::Command;

pub fn get_deps(package: &String) -> Option<std::vec::Vec<String>> {
    let mut packages_dep_vec = Vec::new();

    let output = Command::new("pactree")
        .arg("-l")
        .arg("-u")
        .arg("-r")
        .arg(&package)
        .output()
        .expect("failed");

    if output.status.success() {
        let string_output = String::from_utf8(output.stdout).unwrap();
        let mut dep_packages: Vec<&str> = string_output.split("\n").collect();

        // remove first entry because it is the package itself
        dep_packages.remove(0);
        dep_packages.pop();
        for dep_package in dep_packages {
            match dep_package
                .chars()
                .position(|c| c == '=' || c == '<' || c == '>')
            {
                Some(pos) => {
                    let dep_packages_without_constrains = dep_package.to_string().split_off(pos);
                    packages_dep_vec.push(dep_packages_without_constrains);
                }
                None => packages_dep_vec.push(dep_package.to_string()),
            }
        }
        Some(packages_dep_vec)
    } else {
        None
    }
}
