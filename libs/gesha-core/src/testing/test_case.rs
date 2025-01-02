use crate::Result;
use gesha_rust_shapes::ToRustType;
use openapi_types::v3_0;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::vec;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TestCase {
    /// conversion from OpenAPI v3.0.* to Rust codes
    V3_0_Rust(ConversionSetting<v3_0::ComponentsObject, gesha_rust_types::SourceCode>),
}

impl TestCase {
    pub fn all() -> Vec<Self> {
        vec![Self::V3_0_Rust(ConversionSetting::v3_0_rust("array.yaml"))]
    }
    pub fn require(_path: &str) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ConversionSetting<From, To> {
    pub output: PathBuf,
    pub schema: PathBuf,
    pub example: PathBuf,
    phantom: PhantomData<(From, To)>,
}

const COMPONENTS_PATH: &str = "examples/v3_0/src/components";

impl ConversionSetting<v3_0::ComponentsObject, gesha_rust_types::SourceCode> {
    fn v3_0_rust(yaml_name: &str) -> Self {
        // let yaml_name = yaml_name.into();
        let rs_name = yaml_name.replace(".yaml", ".rs");
        let dir = "schemas";
        Self {
            output: format!("output/v3.0/components/{dir}/{rs_name}").into(),
            schema: format!("{COMPONENTS_PATH}/{dir}/{yaml_name}").into(),
            example: format!("{COMPONENTS_PATH}/{dir}/{rs_name}").into(),
            phantom: Default::default(),
        }
    }
}

pub trait CanConvert<From>: Sized {
    fn convert(x: From) -> Result<Self>;
}

impl CanConvert<v3_0::ComponentsObject> for gesha_rust_types::SourceCode {
    fn convert(x: v3_0::ComponentsObject) -> Result<Self> {
        // TODO: remove unwrap
        let y = ToRustType::apply(x).unwrap();
        Ok(y)
    }
}
