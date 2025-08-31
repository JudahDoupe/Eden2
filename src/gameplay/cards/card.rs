#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Card {
    name: String,
}

impl Card {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
