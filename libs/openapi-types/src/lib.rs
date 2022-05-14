pub mod v3_0;

#[derive(Debug)]
pub enum OpenApiDocument {
    V3_0(v3_0::Document),
}
