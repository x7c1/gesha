mod expand_inline_schemas;
use expand_inline_schemas::expand_inline_schemas;

mod resolve_all_of;
use resolve_all_of::resolve_all_of;

mod resolve_optional_fields;
use resolve_optional_fields::resolve_optional_fields;

mod resolve_type_path;
use resolve_type_path::resolve_type_path;

use crate::conversions::v3_0::to_rust_type::components::ComponentsShapes;
use crate::conversions::Result;

pub fn transform_schemas(shapes: ComponentsShapes) -> Result<ComponentsShapes> {
    let shapes = expand_inline_schemas(shapes)?;
    let shapes = resolve_all_of(shapes)?;
    let shapes = resolve_type_path(shapes)?;
    let shapes = resolve_optional_fields(shapes)?;
    Ok(shapes)
}
