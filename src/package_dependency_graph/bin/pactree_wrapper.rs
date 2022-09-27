use std::process::Command;

pub fn get_deps(packages: std::vec::Vec<String>) -> std::vec::Vec<std::vec::Vec<String>> {
    let mut packages_dep_vec = Vec::new();
    for package in packages {
        let mut package_dep_vec: Vec<String> = Vec::new();
        let mut output = Command::new("pactree")
            .arg("-l")
            .arg("-u")
            .arg(package)
            .spawn()
            .expect("failed");
        let ecode = output.wait().expect("failes to wait");
        if ecode.success() {
            let command_output = output.wait_with_output().unwrap();
            let string_output = String::from_utf8(command_output.stdout).unwrap();
            println!("test {}", string_output);
            package_dep_vec.push(string_output);
        } else {
            package_dep_vec.push("".to_string());
        }
        println!("dep size: {}", package_dep_vec.len());
        packages_dep_vec.push(package_dep_vec);
    }
    return packages_dep_vec;
}
