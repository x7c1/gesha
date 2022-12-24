mod render_enum;

use render_enum::{render_enum, render_enum_variants};
use std::fs::File;

mod render_error;
use render_error::render_error;

mod render_media_type;
use render_media_type::render_media_type;

mod render_request_body;
use render_request_body::render_request_body;

use crate::render;
use crate::renderer::Renderer;
use crate::renderer::Result;
use crate::targets::rust_type::{
    DataType, Definition, DeriveAttribute, Imports, Module, Modules, NewTypeDef, PresetDef,
    StructDef, StructField, StructFieldAttribute,
};
use std::io::Write;

impl Renderer for Modules {
    fn render(self, mut write: File) -> Result<()> {
        self.into_iter()
            .try_for_each(|module| render_module(&mut write, module))
    }
}

fn render_module(mut write: &mut File, module: Module) -> Result<()> {
    render! { write =>
        echo > "pub mod {name}", name = module.name;
        "{}" > render_mod_body => module;
        echo > "\n";
    };
    Ok(())
}

fn render_mod_body(mut write: &mut File, module: Module) -> Result<()> {
    render! { write =>
        call > render_use_statements => module.imports;
        echo > "\n";
    }
    module
        .definitions
        .into_iter()
        .try_for_each(|def| render_definition(write, def))
}

fn render_use_statements<W: Write>(mut write: W, xs: Imports) -> Result<()> {
    for x in xs {
        render! { write =>
            echo > "use {x};\n";
        }
    }
    Ok(())
}

fn render_definition(write: &mut File, x: Definition) -> Result<()> {
    match x {
        Definition::StructDef(x) => render_struct(write, x)?,
        Definition::NewTypeDef(x) => render_newtype(write, x)?,
        Definition::EnumDef(x) => render_enum(write, x)?,
        Definition::PresetDef(x) => render_preset(write, x)?,
        Definition::RequestBodyDef(x) => render_request_body(write, x)?,
        Definition::ModDef(x) => render_module(write, x.into())?,
    };
    Ok(())
}

fn render_struct<W: Write>(mut write: W, x: StructDef) -> Result<()> {
    render! { write =>
        echo > "{comments}", comments = x.header.doc_comments;
        call > render_derive_attrs => &x.derive_attrs;
        echo > "pub struct {name}", name = x.header.name;
        "{}" > render_fields => x.fields;
        echo > "\n";
    };
    Ok(())
}

fn render_derive_attrs<W: Write>(mut write: W, attrs: &[DeriveAttribute]) -> Result<()> {
    render! { write =>
        echo > "#[derive({items})]", items = attrs.join(",");
        echo > "\n";
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
        call > render_field_attrs => field.attributes;
        echo > "pub {name}: ", name = field.name;
        call > render_data_type => &field.data_type;
    };
    Ok(())
}

fn render_field_attrs<W: Write>(mut write: W, attrs: Vec<StructFieldAttribute>) -> Result<()> {
    for attr in attrs.into_iter() {
        render! { write => echo > "#[{attr}]"; }
    }
    Ok(())
}

fn render_data_types<W: Write>(mut write: W, types: &[DataType]) -> Result<()> {
    let items = types
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    render! { write => echo > "{items}"; }
    Ok(())
}

fn render_data_type<W: Write>(mut write: W, data_type: &DataType) -> Result<()> {
    render! { write =>
        echo > "{type_name}", type_name = data_type;
    }
    Ok(())
}

fn render_newtype<W: Write>(mut write: W, x: NewTypeDef) -> Result<()> {
    render! { write =>
        echo > "{comments}", comments = x.header.doc_comments;
        call > render_derive_attrs => &x.derive_attrs;
        echo > "pub struct {name}", name = x.header.name;
        "()" > render_data_type => &x.data_type;
        echo > ";";

        echo >
            "impl From<{data_type}> for {name} {{
                fn from(this: {data_type}) -> Self {{
                    Self(this)
                }}
            }}
            impl From<{name}> for {data_type} {{
                fn from(this: {name}) -> Self {{
                    this.0
                }}
            }}",
            data_type = x.data_type,
            name = x.header.name;

        echo > "\n\n";
    }
    Ok(())
}

fn render_preset<W: Write>(mut write: W, x: PresetDef) -> Result<()> {
    match x {
        PresetDef::Patch => {
            let source = include_str!("patch.rs.tpl");
            render! { write => echo > "{source}"; }
        }
        PresetDef::MediaType(media_type) => {
            render_media_type(write, media_type)?;
        }
        PresetDef::FromJson => {
            render! { write => echo > "
                pub fn from_json<'a, A: Deserialize<'a>>(text: &'a str) -> Result<A> {{
                    serde_json::from_str(text).map_err(Error::InvalidJson)
                }}
            ";}
        }
        PresetDef::Error(e) => {
            render_error(write, e)?;
        }
    }
    Ok(())
}
