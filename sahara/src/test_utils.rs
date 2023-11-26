use crate::{ModuleRegistry, TypeDefinition, TypeId};

pub fn create_type_definition(name: &str) -> TypeDefinition {
    let mut module_registry = ModuleRegistry::new();
    let test_module = module_registry.register("test".to_string());
    TypeDefinition::new(TypeId::new(&test_module, name))
}
