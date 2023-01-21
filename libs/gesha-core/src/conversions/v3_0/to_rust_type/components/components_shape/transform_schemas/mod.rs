mod convert_all_of;
use convert_all_of::convert_all_of;

mod convert_one_of;
use convert_one_of::convert_one_of;

mod expand_inline_schemas;
use expand_inline_schemas::expand_inline_schemas;

mod insert_imports;
use insert_imports::insert_imports;

mod resolve_optional_fields;
use resolve_optional_fields::resolve_optionality;

mod resolve_type_path;
use resolve_type_path::resolve_type_path;

use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;

pub fn transform_schemas(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    shape = expand_inline_schemas(shape)?;
    shape = convert_all_of(shape)?;
    shape = convert_one_of(shape)?;
    shape = resolve_type_path(shape)?;
    shape = resolve_optionality(shape)?;
    shape = insert_imports(shape)?;
    Ok(shape)
}
