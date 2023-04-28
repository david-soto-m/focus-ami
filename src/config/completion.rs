use dialoguer::Completion;

#[derive(Clone, Copy, Debug)]
pub struct Compl {}

impl<'a> Completion for Compl {
    fn get(&self, get: &str) -> Option<String> {
        let a: Vec<_> = get.trim().split_whitespace().collect();
        if a.len() > 1 {
            return None;
        } else if get.starts_with("a") {
            Some(String::from("add"))
        } else if get.starts_with("r") {
            Some(String::from("rm"))
        } else if get.starts_with("d") {
            Some(String::from("diff"))
        } else if get.starts_with("q") {
            Some(String::from("quit"))
        } else if get.starts_with("v") {
            Some(String::from("view"))
        } else {
            None
        }
    }
}
