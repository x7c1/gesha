use crate::render;
use crate::renderer::Renderer;
use crate::renderer::Result;
use crate::targets::rust_type::{
    DataType, Definition, DeriveAttribute, EnumDef, EnumVariant, Module, Modules, NewTypeDef,
    PresetDef, StructDef, StructField, StructFieldAttribute, UseStatement,
};
use std::io::Write;

impl Renderer for Modules {
    fn render<W: Write>(self, mut write: W) -> Result<()> {
        self.into_iter()
            .try_for_each(|module| render_module(&mut write, module))
    }
}

fn render_module<W: Write>(mut write: W, module: Module) -> Result<()> {
    render! { write =>
        echo > "pub mod {name}", name = module.name;
        "{}" > render_mod_body => module;
        echo > "\n";
    };
    Ok(())
}

fn render_mod_body<W: Write>(mut write: W, module: Module) -> Result<()> {
    render! { write =>
        call > render_use_statements => module.use_statements;
        echo > "\n";
    }
    module
        .definitions
        .into_iter()
        .try_for_each(|def| render_definition(&mut write, def))
}

fn render_use_statements<W: Write>(mut write: W, xs: Vec<UseStatement>) -> Result<()> {
    for x in xs {
        render! { write =>
            echo > "use {target};\n", target = String::from(x);
        }
    }
    Ok(())
}

fn render_definition<W: Write>(write: W, x: Definition) -> Result<()> {
    match x {
        Definition::StructDef(x) => render_struct(write, x)?,
        Definition::NewTypeDef(x) => render_newtype(write, x)?,
        Definition::EnumDef(x) => render_enum(write, x)?,
        Definition::PresetDef(x) => render_preset(write, x)?,
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

fn render_enum<W: Write>(mut write: W, x: EnumDef) -> Result<()> {
    render! { write =>
        echo > "{comments}", comments = x.header.doc_comments;
        call > render_derive_attrs => &x.derive_attrs;
        echo > "pub enum {name}", name = x.header.name;
        "{}" > render_enum_variants => x.variants;
        echo > "\n\n";
    }
    Ok(())
}

fn render_enum_variants<W: Write>(mut write: W, variants: Vec<EnumVariant>) -> Result<()> {
    for variant in variants {
        render! { write =>
            echo > "{name},\n", name = variant.to_upper_camel();
        }
    }
    Ok(())
}

fn render_preset<W: Write>(mut write: W, x: PresetDef) -> Result<()> {
    render! { write =>
        echo > "{x}";
    }
    Ok(())
}
