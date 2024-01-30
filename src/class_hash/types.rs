use serde::Serialize;

#[derive(Serialize)]
pub struct Contract {
    name: String,
    sierra: String,
    casm: String,
}

#[derive(Serialize)]
pub struct Contracts {
    pub contracts: Vec<Contract>,
}

impl Contract {
    pub fn new(name: String, sierra: String, casm: String) -> Self {
        Self { name, sierra, casm }
    }
}
