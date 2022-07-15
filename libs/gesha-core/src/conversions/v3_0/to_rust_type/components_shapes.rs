use crate::conversions::v3_0::to_rust_type::DefinitionShape;
use crate::conversions::v3_0::to_rust_type::DefinitionShape::{Fixed, InProcess};
use crate::conversions::Error::PostProcessBroken;
use crate::conversions::Result;
use crate::targets::rust_type::{
    DataType, Definition, Module, ModuleName, Modules, PresetDef, UseStatement,
};
use openapi_types::v3_0::ReferenceObject;

#[derive(Clone, Debug)]
pub struct ComponentsShapes {
    pub(super) schemas: Vec<DefinitionShape>,
}

impl ComponentsShapes {
    pub fn into_modules(self) -> Result<Modules> {
        let modules = create_modules(vec![create_module("schemas", self.schemas)?]);
        Ok(modules)
    }

    pub(super) fn find_definition(&self, object: &ReferenceObject) -> Result<&DefinitionShape> {
        // TODO: support other locations like 'components/responses' etc
        find_shape("#/components/schemas/", &self.schemas, object).ok_or_else(|| unimplemented!())
    }
}

fn create_module<A: Into<String>>(name: A, shapes: Vec<DefinitionShape>) -> Result<Module> {
    let definitions = shapes
        .into_iter()
        .map(to_definition)
        .collect::<Result<Vec<Definition>>>()?;

    let mut imports = default_imports();
    if definitions.iter().any(|x| x.any_type(is_patch)) {
        imports.push(UseStatement::new("super::core::Patch"));
    }
    let module = Module::new(ModuleName::new(name), definitions, imports);
    Ok(module)
}

fn create_modules(modules: Vec<Module>) -> Modules {
    let mut modules = Modules::new(modules);
    if let Some(core) = create_core_module(&modules) {
        modules.push(core);
    }
    modules
}

fn create_core_module(modules: &Modules) -> Option<Module> {
    let mut core_defs = vec![];

    if modules.any_type(is_patch) {
        core_defs.push(PresetDef::patch().into());
    }

    if core_defs.is_empty() {
        None
    } else {
        let module = Module::new(ModuleName::new("core"), core_defs, default_imports());
        Some(module)
    }
}

fn default_imports() -> Vec<UseStatement> {
    vec![
        UseStatement::new("serde::Deserialize"),
        UseStatement::new("serde::Serialize"),
    ]
}

fn is_patch(x: &DataType) -> bool {
    match x {
        DataType::Bool => false,
        DataType::Int32 => false,
        DataType::Int64 => false,
        DataType::Float32 => false,
        DataType::Float64 => false,
        DataType::Option(x) => is_patch(x),
        DataType::Patch(_) => true,
        DataType::String => false,
        DataType::Vec(x) => is_patch(x),
        DataType::Custom(_) => false,
    }
}

fn to_definition(shape: DefinitionShape) -> Result<Definition> {
    match shape {
        Fixed(def) => Ok(def),
        InProcess(process) => Err(PostProcessBroken {
            detail: format!("post-process has been left.\n{:#?}", process),
        }),
    }
}

fn find_shape<'a, 'b>(
    prefix: &str,
    defs: &'a [DefinitionShape],
    target: &'b ReferenceObject,
) -> Option<&'a DefinitionShape> {
    let type_ref = target.as_ref();
    if type_ref.starts_with(prefix) {
        let name = type_ref.replace(prefix, "");
        defs.iter().find(|shape| shape.is_struct_name(&name))
    } else {
        None
    }
}
