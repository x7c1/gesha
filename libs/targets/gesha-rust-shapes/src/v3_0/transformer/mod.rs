mod transform_core;

use gesha_collections::tracking::WithContextOps;
use transform_core::transform_core;

mod transform_request_bodies;
use transform_request_bodies::transform_request_bodies;

mod transform_schemas;
use transform_schemas::transform_schemas;

use crate::v3_0::components::ComponentsShape;
use gesha_core::conversions::{Output, Result};

pub fn transform(shape: ComponentsShape) -> Result<ComponentsShape> {
    let maybe = Output::optionize(transform_schemas)(Some(shape))
        .with_context("schemas")
        .to_result()?;

    let maybe = Output::optionize(transform_request_bodies)(maybe)
        .with_context("request_bodies")
        .to_result()?;

    let shape = Output::optionize(transform_core)(maybe)
        .with_context("core")
        .ok_or_errors()?;

    Ok(shape)
}
