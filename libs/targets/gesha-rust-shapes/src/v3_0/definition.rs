use crate::v3_0::components::core::CoreShape;
use crate::v3_0::components::request_bodies::RequestBodiesShape;
use crate::v3_0::components::schemas::SchemasShape;
use crate::v3_0::components::ComponentsShape;
use gesha_core::conversions;
use gesha_core::conversions::v3_0::{request_bodies_files, schemas_files, COMPONENTS_PATH};
use gesha_core::conversions::{Definition, TestCase, TestSuite};
use gesha_core::Error::FormatFailed;
use gesha_rust_types::{ModuleDeclarations, ModuleName, NonDocComments};
use openapi_types::v3_0;
use std::path::Path;
use std::process::Command;

pub struct ComponentsToRustTypes;

impl Definition for ComponentsToRustTypes {
    type OpenApiType = v3_0::ComponentsObject;
    type TargetType = gesha_rust_types::SourceCode;

    fn convert(this: Self::OpenApiType) -> Result<Self::TargetType, conversions::Error> {
        let shapes = ComponentsShape {
            schemas: SchemasShape::shape(this.schemas)?,
            request_bodies: RequestBodiesShape::shape(this.request_bodies)?,
            core: CoreShape::default(),
        };
        let mod_defs = shapes.into_mod_defs()?;
        Ok(new_code().set_mod_defs(mod_defs))
    }

    fn format_code(path: &Path) -> gesha_core::Result<String> {
        let output = Command::new("rustfmt")
            .arg("--verbose")
            .arg(path)
            .output()
            .map_err(|e| FormatFailed {
                path: path.into(),
                detail: format!("{:?}", e),
            })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(FormatFailed {
                path: path.into(),
                detail: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }

    fn test_suites() -> Vec<TestSuite<Self>> {
        vec![
            create_suite(schemas_files(), "schemas"),
            create_suite(request_bodies_files(), "request_bodies"),
        ]
    }

    fn test_suites_content(suite: &TestSuite<Self>) -> Self::TargetType {
        let decls = suite
            .test_cases
            .iter()
            .map(|case| case.module_name.clone())
            .map(ModuleName::new)
            .collect::<ModuleDeclarations>();

        new_code().set_mod_decls(decls)
    }
}

fn new_code() -> gesha_rust_types::SourceCode {
    gesha_rust_types::SourceCode::empty().set_preamble(NonDocComments::block(
        "    Generated by gesha command; DO NOT EDIT BY HAND!",
    ))
}

fn create_suite(filenames: Vec<&str>, parent_name: &str) -> TestSuite<ComponentsToRustTypes> {
    let enclosed_cases = filenames
        .iter()
        .map(|filename| to_test_case(parent_name, filename))
        .collect();

    TestSuite {
        mod_path: format!("{COMPONENTS_PATH}/{parent_name}.rs").into(),
        test_cases: enclosed_cases,
    }
}

fn to_test_case(parent_name: &str, yaml_name: &str) -> TestCase<ComponentsToRustTypes> {
    let rs_name = yaml_name.replace(".yaml", ".rs");
    TestCase {
        output: format!("output/v3.0/components/{parent_name}/{rs_name}").into(),
        schema: format!("{COMPONENTS_PATH}/{parent_name}/{yaml_name}").into(),
        example: format!("{COMPONENTS_PATH}/{parent_name}/{rs_name}").into(),
        module_name: yaml_name.replace(".yaml", ""),
        phantom: Default::default(),
    }
}
