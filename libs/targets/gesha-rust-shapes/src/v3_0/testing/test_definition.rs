use crate::v3_0::converter::new_code;
use crate::v3_0::testing::ComponentsConverter;
use gesha_core::testing::v3_0::{COMPONENTS_PATH, request_bodies_files, schemas_files};
use gesha_core::testing::{TestCase, TestCaseIndex, TestDefinition};
use gesha_rust_types::{ModuleDeclarations, ModuleName};

impl TestDefinition for ComponentsConverter {
    fn list_indexes(&self) -> Vec<TestCaseIndex<Self>> {
        vec![
            create_index(schemas_files(), "schemas"),
            create_index(request_bodies_files(), "request_bodies"),
        ]
    }

    fn generate_index_code(&self, index: &TestCaseIndex<Self>) -> Self::TargetType {
        let decls = index
            .test_cases
            .iter()
            .map(|case| case.module_name.clone())
            .map(ModuleName::new)
            .collect::<ModuleDeclarations>();

        new_code().set_mod_decls(decls)
    }
}

fn create_index(filenames: Vec<&str>, parent_name: &str) -> TestCaseIndex<ComponentsConverter> {
    let enclosed_cases = filenames
        .iter()
        .map(|filename| to_test_case(parent_name, filename))
        .collect();

    TestCaseIndex {
        mod_path: format!("{COMPONENTS_PATH}/{parent_name}.rs").into(),
        test_cases: enclosed_cases,
    }
}

fn to_test_case(parent_name: &str, yaml_name: &str) -> TestCase<ComponentsConverter> {
    let rs_name = yaml_name.replace(".yaml", ".rs");
    TestCase {
        output: format!("output/v3.0/components/{parent_name}/{rs_name}").into(),
        schema: format!("{COMPONENTS_PATH}/{parent_name}/{yaml_name}").into(),
        example: format!("{COMPONENTS_PATH}/{parent_name}/{rs_name}").into(),
        module_name: yaml_name.replace(".yaml", ""),
        phantom: Default::default(),
    }
}
