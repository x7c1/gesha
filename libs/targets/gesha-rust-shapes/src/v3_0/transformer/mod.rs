mod transform_core;
use transform_core::transform_core;

mod transform_request_bodies;
use transform_request_bodies::transform_request_bodies;

mod transform_schemas;
use transform_schemas::transform_schemas;

use crate::v3_0::components::core::ComponentsShape;
use gesha_core::conversions::{Output, Result, with_key};

pub fn transform(shape: ComponentsShape) -> Result<ComponentsShape> {
    let maybe = Output::optionize(transform_schemas)(Some(shape))
        .bind_errors(with_key("schemas"))
        .to_result()?;

    let maybe = Output::optionize(transform_request_bodies)(maybe)
        .bind_errors(with_key("request_bodies"))
        .to_result()?;

    let shape = Output::optionize(transform_core)(maybe)
        .bind_errors(with_key("core"))
        .ok_or_errors()?;

    Ok(shape)
}
