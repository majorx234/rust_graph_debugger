pub fn read_package_list() -> (std::vec::Vec<String>) {
    let mut package_list: std::vec::Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(size) => {
                if size != 0 {
                    if line.ends_with('\n') {
                        line.pop();
                    }
                    package_list.push(line)
                } else {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    package_list
}
