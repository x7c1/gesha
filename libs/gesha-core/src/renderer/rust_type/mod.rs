use crate::render;
use crate::renderer::Renderer;
use crate::renderer::Result;
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
    render! { write =>
        echo > "pub mod {name}";
        "{}" > render_definitions => definitions;
    };
    Ok(())
}

fn render_definitions<W: Write>(mut write: W, xs: Vec<Definition>) -> Result<()> {
    xs.into_iter()
        .try_for_each(|def| render_definition(&mut write, def))
}

fn render_definition<W: Write>(write: W, x: Definition) -> Result<()> {
    match x {
        Definition::StructDef(x) => render_struct(write, x)?,
        Definition::VecDef(_x) => unimplemented!(),
    };
    Ok(())
}

fn render_struct<W: Write>(mut write: W, x: StructDef) -> Result<()> {
    render! { write =>
        echo > "pub struct {name}", name = x.name;
        "{}" > render_fields => x.fields;
    };
    Ok(())
}

fn render_fields<W: Write>(mut write: W, fields: Vec<StructField>) -> Result<()> {
    for field in fields.into_iter() {
        render! { write =>
            call > render_field => field;
            echo > ",\n";
        };
    }
    Ok(())
}

fn render_field<W: Write>(mut write: W, field: StructField) -> Result<()> {
    render! { write =>
        echo > "pub {name}: ", name = field.name;
        call > render_field_type => field.data_type;
    };
    Ok(())
}

fn render_field_type<W: Write>(mut write: W, field_type: FieldType) -> Result<()> {
    fn from_type(x: FieldType) -> String {
        match x {
            FieldType::String => "String".to_string(),
            FieldType::Int32 => "i32".to_string(),
            FieldType::Int64 => "i64".to_string(),
            FieldType::Vec => unimplemented!(),
            FieldType::Option(x) => format!("Option<{}>", from_type(*x)),
        }
    }
    render! { write =>
        echo > "{type_name}", type_name = from_type(field_type);
    }
    Ok(())
}
