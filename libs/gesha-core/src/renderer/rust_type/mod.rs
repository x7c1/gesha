use crate::render;
use crate::renderer::Renderer;
use crate::renderer::Result;
use crate::targets::rust_type::{
    DataType, Definition, ModuleName, Modules, NewTypeDef, StructDef, StructField,
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
        Definition::NewTypeDef(x) => render_newtype(write, x)?,
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
        call > render_data_type => field.data_type;
    };
    Ok(())
}

fn render_data_type<W: Write>(mut write: W, data_type: DataType) -> Result<()> {
    render! { write =>
        echo > "{type_name}", type_name = String::from(data_type);
    }
    Ok(())
}

fn render_newtype<W: Write>(mut write: W, x: NewTypeDef) -> Result<()> {
    render! { write =>
        echo > "pub struct {name}", name = x.name;
        "()" > render_data_type => x.data_type;
        echo > ";\n";
    }
    Ok(())
}
