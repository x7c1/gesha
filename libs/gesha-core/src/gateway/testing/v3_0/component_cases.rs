use crate::gateway;
use crate::gateway::testing::v3_0::ComponentKind;
use crate::gateway::testing::TestCase;
use crate::gateway::Error::UnsupportedExampleLocation;
use crate::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::borrow::Cow;

const COMPONENTS_PATH: &str = "examples/v3.0/src/components";

pub struct ComponentCase {
    inner: TestCase<(v3_0::ComponentsObject, Modules)>,
}

impl ComponentCase {
    fn new<A>(kind: ComponentKind, yaml_name: A) -> ComponentCase
    where
        A: Into<Cow<'static, str>>,
    {
        let yaml_name = yaml_name.into();
        let rs_name = yaml_name.replace(".yaml", ".rs");
        let dir = kind.name();
        let inner = TestCase {
            output: format!("output/v3.0/components/{dir}/{rs_name}").into(),
            schema: format!("{COMPONENTS_PATH}/{dir}/{yaml_name}").into(),
            example: format!("{COMPONENTS_PATH}/{dir}/{rs_name}").into(),
            module_name: yaml_name.replace(".yaml", ""),
            phantom: Default::default(),
        };
        Self { inner }
    }

    pub fn from_path(path: String) -> gateway::Result<Self> {
        let to_case = |kind: ComponentKind| {
            let dir_path = format!("{COMPONENTS_PATH}/{dir}/", dir = kind.name());
            path.starts_with(&dir_path).then(|| {
                let yaml_name = path.replace(&dir_path, "");
                ComponentCase::new(kind, yaml_name)
            })
        };
        ComponentKind::all()
            .into_iter()
            .find_map(to_case)
            .ok_or_else(|| UnsupportedExampleLocation(path.to_owned()))
    }
}

impl From<ComponentCase> for TestCase<(v3_0::ComponentsObject, Modules)> {
    fn from(this: ComponentCase) -> Self {
        this.inner
    }
}

pub struct ComponentCases {
    pub module_path: String,
    cases: Vec<ComponentCase>,
}

impl ComponentCases {
    pub fn from_vec<A: Into<Cow<'static, str>>>(kind: ComponentKind, yaml_names: Vec<A>) -> Self {
        let cases = yaml_names
            .into_iter()
            .map(|a| a.into())
            .map(|yaml_name| ComponentCase::new(kind, yaml_name))
            .collect();

        let module_path = format!(
            "examples/v3.0/src/components/{module}.rs",
            module = kind.name()
        );

        Self { module_path, cases }
    }
}

impl From<ComponentCases> for Vec<TestCase<(v3_0::ComponentsObject, Modules)>> {
    fn from(this: ComponentCases) -> Self {
        this.cases.into_iter().map(|x| x.inner).collect()
    }
}
