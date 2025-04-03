mod collapse_single_all_of;
use collapse_single_all_of::collapse_single_all_of;

mod collapse_single_one_of;
use collapse_single_one_of::collapse_single_one_of;

mod convert_all_of;
use convert_all_of::convert_all_of;

mod convert_one_of;
use convert_one_of::convert_one_of;

mod definition_transformer;
use definition_transformer::DefinitionTransformer;

mod expand_inline_schemas;
use expand_inline_schemas::expand_inline_schemas;
use gesha_collections::tracking::WithKeyOps;

mod insert_impl_serde_macro;
use insert_impl_serde_macro::insert_impl_serde_macro;

mod insert_imports;
use insert_imports::insert_imports;

mod resolve_optional_fields;
use resolve_optional_fields::resolve_optionality;

mod resolve_type_path;
use resolve_type_path::resolve_type_path;

use crate::v3_0::components::ComponentsShape;
use gesha_core::conversions::Result;

pub fn transform_schemas(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    shape = collapse_single_all_of(shape).with_key("#(collapse_single_all_of)")?;
    shape = collapse_single_one_of(shape).with_key("#(collapse_single_one_of)")?;
    shape = expand_inline_schemas(shape).with_key("#(expand_inline_schemas)")?;
    shape = convert_all_of(shape).with_key("#(convert_all_of)")?;
    shape = convert_one_of(shape).with_key("#(convert_one_of)")?;
    shape = resolve_type_path(shape).with_key("#(resolve_type_path)")?;
    shape = resolve_optionality(shape).with_key("#(resolve_optionality)")?;
    shape = insert_imports(shape).with_key("#(insert_imports)")?;
    shape = insert_impl_serde_macro(shape).with_key("#(insert_impl_serde_macro)")?;
    Ok(shape)
}
