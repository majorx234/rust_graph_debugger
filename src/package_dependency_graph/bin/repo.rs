use crate::dependency::Dependency;
use crate::pactree_wrapper::get_deps;
pub struct Repo {}

impl Dependency for Repo {
    fn get_deps(&self, item: &String) -> Option<Vec<String>> {
        get_deps(item)
    }
}
