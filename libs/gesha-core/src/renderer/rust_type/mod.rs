use crate::renderer::Error::CannotWrite;
use crate::renderer::Result;
use crate::renderer::{render, Renderer};
use crate::targets::rust_type::{
    Definition, FieldType, ModuleName, Modules, StructDef, StructField,
};
use std::io::Write;

impl Renderer for Modules {
    fn render<W: Write>(self, mut write: W) -> Result<()> {
        self.into_iter()
            .try_for_each(|(name, defs)| render_module(&mut write, name, defs))
    }
}

fn render_module<W: Write>(
    mut write: W,
    name: ModuleName,
    definitions: Vec<Definition>,
) -> Result<()> {
    render!(write, "pub mod {name}", { definitions });
    Ok(())
}

impl Renderer for Vec<Definition> {
    fn render<W: Write>(self, mut write: W) -> Result<()> {
        let x = render_items(self, render_definition)?;
        write.write(x.as_bytes()).map(|_| ()).map_err(CannotWrite)
    }
}

// TODO: receive Write
fn render_definition(x: Definition) -> Result<String> {
    match x {
        Definition::StructDef(x) => render_struct(x),
        Definition::VecDef(_x) => unimplemented!(),
    }
}

fn render_struct(x: StructDef) -> Result<String> {
    Ok(format!(
        "pub struct {name} {{\n{fields}\n}}\n",
        name = x.name,
        fields = render_fields(x.fields)?
    ))
}

fn render_fields(fields: Vec<StructField>) -> Result<String> {
    let rendered = fields
        .into_iter()
        .map(render_field)
        .collect::<Result<Vec<String>>>()?
        .join(",\n");

    Ok(rendered)
}

fn render_field(field: StructField) -> Result<String> {
    Ok(format!(
        "pub {name}: {type_name}",
        name = field.name,
        type_name = render_field_type(field.data_type)?
    ))
}

fn render_field_type(field_type: FieldType) -> Result<String> {
    let type_name = match field_type {
        FieldType::String => "String".to_string(),
        FieldType::Int64 => "i64".to_string(),
        FieldType::Vec => "Vec<???>".to_string(),
    };
    Ok(type_name)
}

fn render_items<A, B, F>(items: A, f: F) -> Result<String>
where
    A: IntoIterator<Item = B>,
    F: FnMut(B) -> Result<String>,
{
    let rendered = items
        .into_iter()
        .map(f)
        .collect::<Result<Vec<String>>>()?
        .join("\n");

    Ok(rendered)
}
