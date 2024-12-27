mod macros;
use crate::render;

mod render_enum;
use render_enum::{render_enum, render_enum_variants};

mod render_error;
use render_error::render_error;

mod render_media_type;
use render_media_type::render_media_type;

mod render_request_body;
use render_request_body::render_request_body;

use crate::{
    DataType, Definition, DeriveAttribute, Imports, ModDef, Modules, NewTypeDef, PresetDef,
    SerdeAttribute, StructDef, StructField, StructFieldAttribute, TypeHeader,
};

use std::fmt;
use std::fmt::{Display, Write};

impl Display for Modules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().try_for_each(|module| render_module(f, module))
    }
}

fn render_module(write: &mut impl Write, module: &ModDef) -> fmt::Result {
    render! { write =>
        echo > "pub mod {name}", name = module.name;
        "{}" > render_mod_body => module;
        echo > "\n";
    };
    Ok(())
}

fn render_mod_body(write: &mut impl Write, module: &ModDef) -> fmt::Result {
    render! { write =>
        call > render_use_statements => &module.imports;
        echo > "\n";
    }
    module
        .defs
        .iter()
        .try_for_each(|def| render_definition(write, def))
}

fn render_use_statements(write: &mut impl Write, xs: &Imports) -> fmt::Result {
    for x in xs.iter() {
        render! { write =>
            echo > "use {x};\n";
        }
    }
    Ok(())
}

fn render_definition(write: &mut impl Write, x: &Definition) -> fmt::Result {
    match x {
        Definition::StructDef(x) => render_struct(write, x)?,
        Definition::NewTypeDef(x) => render_newtype(write, x)?,
        Definition::EnumDef(x) => render_enum(write, x)?,
        Definition::PresetDef(x) => render_preset(write, x)?,
        Definition::RequestBodyDef(x) => render_request_body(write, x)?,
        Definition::ModDef(x) => render_module(write, x)?,
    };
    Ok(())
}

fn render_header(write: &mut impl Write, x: &TypeHeader) -> fmt::Result {
    render! { write =>
        echo > "{comments}", comments = x.doc_comments;
        call > render_derive_attrs => &x.derive_attrs;
        call > render_serde_attrs => &x.serde_attrs;
    }
    Ok(())
}

fn render_struct(write: &mut impl Write, x: &StructDef) -> fmt::Result {
    render! { write =>
        call > render_header => &x.header;
        echo > "pub struct {name}", name = x.header.name;
        "{}" > render_fields => &x.fields;
        echo > "\n";
    };
    Ok(())
}

fn render_derive_attrs(write: &mut impl Write, attrs: &[DeriveAttribute]) -> fmt::Result {
    render! { write =>
        echo > "#[derive({items})]", items = attrs.join(",");
        echo > "\n";
    };
    Ok(())
}

fn render_serde_attrs(write: &mut impl Write, attrs: &[SerdeAttribute]) -> fmt::Result {
    if attrs.is_empty() {
        return Ok(());
    }
    render! { write =>
        echo > "#[serde({items})]", items = attrs.join(",");
        echo > "\n";
    };
    Ok(())
}

fn render_fields(write: &mut impl Write, fields: &[StructField]) -> fmt::Result {
    for field in fields {
        render! { write =>
            call > render_field => field;
            echo > ",\n";
        };
    }
    Ok(())
}

fn render_field(write: &mut impl Write, field: &StructField) -> fmt::Result {
    render! { write =>
        call > render_field_attrs => &field.attributes;
        echo > "pub {name}: ", name = field.name;
        call > render_data_type => &field.data_type;
    };
    Ok(())
}

fn render_field_attrs(write: &mut impl Write, attrs: &[StructFieldAttribute]) -> fmt::Result {
    for attr in attrs.iter() {
        render! { write => echo > "#[{attr}]"; }
    }
    Ok(())
}

fn render_data_types(write: &mut impl Write, types: &[DataType]) -> fmt::Result {
    let items = types
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    render! { write => echo > "{items}"; }
    Ok(())
}

fn render_data_type(write: &mut impl Write, data_type: &DataType) -> fmt::Result {
    render! { write =>
        echo > "{type_name}", type_name = data_type;
    }
    Ok(())
}

fn render_newtype(write: &mut impl Write, x: &NewTypeDef) -> fmt::Result {
    render! { write =>
        call > render_header => &x.header;
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

fn render_preset(write: &mut impl Write, x: &PresetDef) -> fmt::Result {
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
