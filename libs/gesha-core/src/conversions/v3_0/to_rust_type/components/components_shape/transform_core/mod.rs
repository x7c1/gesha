use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::v3_0::to_rust_type::components::ComponentsShape;
use crate::conversions::Result;

pub fn transform_core(shapes: ComponentsShape) -> Result<ComponentsShape> {
    // TODO
    println!("transform_core: {:#?}", shapes);
    Ok(shapes)
}
