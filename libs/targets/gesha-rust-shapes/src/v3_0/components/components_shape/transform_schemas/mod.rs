mod collapse_single_all_of;
use collapse_single_all_of::collapse_single_all_of;

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

use crate::v3_0::components::ComponentsShape;
use gesha_core::conversions::{by_key, Result};

pub fn transform_schemas(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    shape = collapse_single_all_of(shape).map_err(by_key("#(collapse_single_all_of)"))?;
    shape = expand_inline_schemas(shape).map_err(by_key("#(expand_inline_schemas)"))?;
    shape = convert_all_of(shape).map_err(by_key("#(convert_all_of)"))?;
    shape = convert_one_of(shape).map_err(by_key("#(convert_one_of)"))?;
    shape = resolve_type_path(shape).map_err(by_key("#(resolve_type_path)"))?;
    shape = resolve_optionality(shape).map_err(by_key("#(resolve_optionality)"))?;
    shape = insert_imports(shape).map_err(by_key("#(insert_imports)"))?;
    Ok(shape)
}
