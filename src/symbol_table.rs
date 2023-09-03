#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: &str) -> Identifier {
        Identifier {
            name: String::from(name),
        }
    }
}
