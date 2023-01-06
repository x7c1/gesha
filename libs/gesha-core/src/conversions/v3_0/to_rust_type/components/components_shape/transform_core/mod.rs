use crate::conversions::v3_0::to_rust_type::components::core::CoreShape;
use crate::conversions::v3_0::to_rust_type::components::ComponentsShapes;
use crate::conversions::Result;

pub fn transform_core(shapes: ComponentsShapes) -> Result<ComponentsShapes> {
    // TODO
    println!("transform_core: {:#?}", shapes);
    Ok(shapes)
}
