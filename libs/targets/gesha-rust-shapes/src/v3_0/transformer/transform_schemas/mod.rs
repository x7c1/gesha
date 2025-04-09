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
use gesha_collections::tracking::WithContextOps;

mod insert_imports;
use insert_imports::insert_imports;

mod insert_macro_for_from;
use insert_macro_for_from::insert_macro_for_from;

mod insert_macro_for_serde;
use insert_macro_for_serde::insert_macro_for_serde;

mod resolve_optional_fields;
use resolve_optional_fields::resolve_optionality;

mod resolve_type_path;
use resolve_type_path::resolve_type_path;

use crate::v3_0::components::ComponentsShape;
use gesha_core::conversions::Result;

pub fn transform_schemas(mut shape: ComponentsShape) -> Result<ComponentsShape> {
    shape = collapse_single_all_of(shape).with_context("#(collapse_single_all_of)")?;
    shape = collapse_single_one_of(shape).with_context("#(collapse_single_one_of)")?;

    shape = expand_inline_schemas(shape).with_context("#(expand_inline_schemas)")?;
    shape = convert_all_of(shape).with_context("#(convert_all_of)")?;
    shape = convert_one_of(shape).with_context("#(convert_one_of)")?;

    shape = resolve_type_path(shape).with_context("#(resolve_type_path)")?;
    shape = resolve_optionality(shape).with_context("#(resolve_optionality)")?;
    shape = insert_imports(shape).with_context("#(insert_imports)")?;

    shape = insert_macro_for_serde(shape).with_context("#(insert_macro_for_serde)")?;
    shape = insert_macro_for_from(shape).with_context("#(insert_macro_for_from)")?;
    Ok(shape)
}
