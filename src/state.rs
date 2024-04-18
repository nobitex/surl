use std::collections::HashMap;

use ethers::abi;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Variable {
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct State {
    pub variables_by_name: HashMap<String, Variable>,
    pub abis_by_name: HashMap<String, abi::Abi>,
}

impl State {
    pub fn new() -> State {
        State {
            variables_by_name: HashMap::new(),
            abis_by_name: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables_by_name.insert(name, Variable { value });
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables_by_name.get(name)
    }

    pub fn set_abi(&mut self, name: String, abi: abi::Abi) {
        self.abis_by_name.insert(name, abi);
    }

    pub fn get_abi(&self, name: &str) -> Option<&abi::Abi> {
        self.abis_by_name.get(name)
    }

    pub fn get_abis(&self) -> &HashMap<String, abi::Abi> {
        &self.abis_by_name
    }

    pub fn save_state(&self, path: &str) {
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer(&file, self).unwrap();
    }

    pub fn load_state(&mut self, path: &str) {
        let file = std::fs::File::open(path).unwrap();
        *self = serde_json::from_reader(file).unwrap();
    }
}

#[test]
fn test_state() {
    let mut state = State::new();
    state.set_variable("foo".to_string(), "bar".to_string());
    assert_eq!(state.get_variable("foo").unwrap().value, "bar");
    assert_eq!(state.get_variable("foo").unwrap().value, "bar");
}
