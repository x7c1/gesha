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
        for def in self.into_iter() {
            render_definition(&mut write, def)?;
        }
        Ok(())
    }
}

fn render_definition<W: Write>(write: W, x: Definition) -> Result<()> {
    match x {
        Definition::StructDef(x) => render_struct(write, x)?,
        Definition::VecDef(_x) => unimplemented!(),
    };
    Ok(())
}

fn render_struct<W: Write>(mut write: W, x: StructDef) -> Result<()> {
    let name = x.name;
    let fields = x.fields;
    render!(write, "pub struct {name}", { fields });
    Ok(())
}

impl Renderer for Vec<StructField> {
    fn render<W: Write>(self, mut write: W) -> Result<()> {
        for field in self.into_iter() {
            render_field(&mut write, field)?;
            writeln!(write, ",")?;
        }
        Ok(())
    }
}

fn render_field<W: Write>(mut write: W, field: StructField) -> Result<()> {
    let name = field.name;
    write!(write, "pub {name}: ")?;
    render_field_type(write, field.data_type)
}

fn render_field_type<W: Write>(mut write: W, field_type: FieldType) -> Result<()> {
    let type_name = match field_type {
        FieldType::String => "String".to_string(),
        FieldType::Int64 => "i64".to_string(),
        FieldType::Vec => "Vec<???>".to_string(),
    };
    write!(write, "{}", type_name)?;
    Ok(())
}
