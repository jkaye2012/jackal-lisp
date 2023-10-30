use std::collections::HashSet;

use crate::FunctionId;

pub struct ModuleName<'a> {
    name: &'a str,
}

impl<'a> ModuleName<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn function_id(&self, function_name: &str) -> FunctionId {
        FunctionId::new(self, function_name)
    }
}

pub struct ModuleRegistry {
    modules: HashSet<String>,
}

impl ModuleRegistry {
    pub fn new() -> ModuleRegistry {
        ModuleRegistry {
            modules: HashSet::new(),
        }
    }

    pub fn register(&mut self, module_name: String) -> ModuleName {
        if self.modules.contains(&module_name) {
            panic!("Attempted to register duplicate module: {}", module_name);
        }

        let clone = module_name.clone();
        self.modules.insert(module_name);
        ModuleName {
            name: self.modules.get(&clone).unwrap(),
        }
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
