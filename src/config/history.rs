use dialoguer::History;

pub struct MyHist {
    hist: Vec<String>,
}
impl MyHist {
    pub fn new() -> Self {
        Self { hist: vec![] }
    }
}

impl History<String> for MyHist {
    fn read(&self, pos: usize) -> Option<String> {
        self.hist
            .get(
                self.hist
                    .len()
                    .checked_sub(pos)
                    .and_then(|x| x.checked_sub(1))
                    .unwrap_or(0),
            )
            .cloned()
    }
    fn write(&mut self, val: &String) {
        self.hist.push(val.to_string());
    }
}
