use std::collections::HashMap;

pub struct VpsConfig {
    vps: HashMap<String, Vps>,
}

struct Vps {
    address: String,
}

impl VpsConfig {
    pub fn new() -> Self {
        Self {
            vps: HashMap::new(),
        }
    }
    pub fn add_vps(&mut self, name: String, address: String) -> &mut Self {
        self.vps.insert(name, Vps { address });
        self
    }
    pub fn get_vps(&self, name: &str) -> Option<&Vps> {
        self.vps.get(name)
    }
    pub fn remove_vps(&mut self, name: &str) -> Option<Vps> {
        self.vps.remove(name)
    }
}
