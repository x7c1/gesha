use crate::renderer::Renderer;
use crate::targets::rust_type::{
    Definition, FieldType, ModuleName, Modules, StructDef, StructField,
};

impl Renderer for Modules {
    fn render(self) -> crate::Result<String> {
        render_items(self, render_module)
    }
}

fn render_module(pair: (ModuleName, Vec<Definition>)) -> crate::Result<String> {
    let (module_name, definitions) = pair;
    let rendered = format!(
        "pub mod {name} {{\n{defs}\n}}",
        name = module_name,
        defs = definitions.render()?,
    );
    Ok(rendered)
}

impl Renderer for Vec<Definition> {
    fn render(self) -> crate::Result<String> {
        render_items(self, render_definition)
    }
}

fn render_definition(x: Definition) -> crate::Result<String> {
    match x {
        Definition::StructDef(x) => render_struct(x),
        Definition::VecDef(_x) => unimplemented!(),
    }
}

fn render_struct(x: StructDef) -> crate::Result<String> {
    println!("def: {:?}", x);
    Ok(format!(
        "pub struct {name} {{\n{fields}\n}}\n",
        name = x.name,
        fields = render_fields(x.fields)?
    ))
}

fn render_fields(fields: Vec<StructField>) -> crate::Result<String> {
    let rendered = fields
        .into_iter()
        .map(render_field)
        .collect::<crate::Result<Vec<String>>>()?
        .join(",\n");

    Ok(rendered)
}

fn render_field(field: StructField) -> crate::Result<String> {
    Ok(format!(
        "pub {name}: {type_name}",
        name = field.name,
        type_name = render_field_type(field.data_type)?
    ))
}

fn render_field_type(field_type: FieldType) -> crate::Result<String> {
    let type_name = match field_type {
        FieldType::String => "String".to_string(),
        FieldType::Int64 => "i64".to_string(),
        FieldType::Vec => "Vec<???>".to_string(),
    };
    Ok(type_name)
}

fn render_items<A, B, F>(items: A, f: F) -> crate::Result<String>
where
    A: IntoIterator<Item = B>,
    F: FnMut(B) -> crate::Result<String>,
{
    let rendered = items
        .into_iter()
        .map(f)
        .collect::<crate::Result<Vec<String>>>()?
        .join("\n");

    Ok(rendered)
}
