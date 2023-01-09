mod expand_inline_schemas;
use expand_inline_schemas::expand_inline_schemas;

mod insert_imports;
use insert_imports::insert_imports;

mod resolve_all_of;
use resolve_all_of::resolve_all_of;

mod resolve_optional_fields;
use resolve_optional_fields::resolve_optionality;

mod resolve_type_path;
use resolve_type_path::resolve_type_path;

use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;

pub fn transform_schemas(shapes: ComponentsShape) -> Result<ComponentsShape> {
    let shapes = expand_inline_schemas(shapes)?;
    let shapes = resolve_all_of(shapes)?;
    let shapes = resolve_type_path(shapes)?;
    let shapes = resolve_optionality(shapes)?;
    let shapes = insert_imports(shapes)?;
    Ok(shapes)
}
