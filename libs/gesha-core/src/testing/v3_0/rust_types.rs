use crate::testing::v3_0::COMPONENTS_PATH;
use crate::testing::{CanConvert, ConversionSetting};
use gesha_rust_shapes::ToRustType;
use openapi_types::v3_0;

impl CanConvert<v3_0::ComponentsObject> for gesha_rust_types::SourceCode {
    fn convert(x: v3_0::ComponentsObject) -> crate::Result<Self> {
        // TODO: remove unwrap
        let y = ToRustType::apply(x).unwrap();
        Ok(y)
    }
}

impl ConversionSetting<v3_0::ComponentsObject, gesha_rust_types::SourceCode> {
    pub fn v3_0_rust(yaml_name: &str) -> Self {
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
