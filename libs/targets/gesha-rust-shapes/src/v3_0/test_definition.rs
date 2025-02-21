use crate::v3_0::Converter;
use gesha_core::testing::v3_0::{request_bodies_files, schemas_files, COMPONENTS_PATH};
use gesha_core::testing::{TestCase, TestDefinition, TestSuite};
use gesha_rust_types::{ModuleDeclarations, ModuleName};

impl TestDefinition for Converter {
    fn test_suites(&self) -> Vec<TestSuite<Self>> {
        vec![
            create_suite(schemas_files(), "schemas"),
            create_suite(request_bodies_files(), "request_bodies"),
        ]
    }

    fn test_suite_code(&self, suite: &TestSuite<Self>) -> Self::TargetType {
        let decls = suite
            .test_cases
            .iter()
            .map(|case| case.module_name.clone())
            .map(ModuleName::new)
            .collect::<ModuleDeclarations>();

        crate::v3_0::converter::new_code().set_mod_decls(decls)
    }
}

fn create_suite(filenames: Vec<&str>, parent_name: &str) -> TestSuite<Converter> {
    let enclosed_cases = filenames
        .iter()
        .map(|filename| to_test_case(parent_name, filename))
        .collect();

    TestSuite {
        mod_path: format!("{COMPONENTS_PATH}/{parent_name}.rs").into(),
        test_cases: enclosed_cases,
    }
}

fn to_test_case(parent_name: &str, yaml_name: &str) -> TestCase<Converter> {
    let rs_name = yaml_name.replace(".yaml", ".rs");
    TestCase {
        output: format!("output/v3.0/components/{parent_name}/{rs_name}").into(),
        schema: format!("{COMPONENTS_PATH}/{parent_name}/{yaml_name}").into(),
        example: format!("{COMPONENTS_PATH}/{parent_name}/{rs_name}").into(),
        module_name: yaml_name.replace(".yaml", ""),
        phantom: Default::default(),
    }
}
