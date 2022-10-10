pub trait Dependency {
    fn get_deps(&self, item: &String) -> Option<Vec<String>>;
}
