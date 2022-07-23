use crate::gateway;
use crate::gateway::testing::TestCase;
use crate::gateway::Error::UnsupportedExampleLocation;
use crate::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::borrow::Cow;

pub enum ComponentsCase {
    Schemas,
}

impl ComponentsCase {
    pub fn all() -> Vec<ComponentsCase> {
        vec![Self::Schemas]
    }

    pub fn to_cases<A>(
        &self,
        yaml_names: Vec<A>,
    ) -> Vec<TestCase<(v3_0::ComponentsObject, Modules)>>
    where
        A: Into<Cow<'static, str>>,
    {
        yaml_names
            .into_iter()
            .map(|a| a.into())
            .map(|yaml_name| self.to_case(yaml_name))
            .collect()
    }

    pub fn path_to_case(
        path: String,
    ) -> gateway::Result<TestCase<(v3_0::ComponentsObject, Modules)>> {
        let this = Self::from(&path)?;
        let yaml_name = path.replace(&this.example_dir_path(), "");
        Ok(this.to_case(yaml_name))
    }

    fn from(path: &str) -> gateway::Result<Self> {
        Self::all()
            .into_iter()
            .find_map(|this| {
                let prefix = this.example_dir_path();
                path.starts_with(&prefix).then(|| this)
            })
            .ok_or_else(|| UnsupportedExampleLocation(path.to_owned()))
    }

    fn dir_name(&self) -> &str {
        match self {
            ComponentsCase::Schemas => "schemas",
        }
    }

    fn example_dir_path(&self) -> String {
        format!("examples/v3.0/src/components/{dir}/", dir = self.dir_name())
    }

    fn to_case<A>(&self, yaml_name: A) -> TestCase<(v3_0::ComponentsObject, Modules)>
    where
        A: Into<Cow<'static, str>>,
    {
        let yaml_name = yaml_name.into();
        let rs_name = yaml_name.replace(".yaml", ".rs");
        let dir = self.dir_name();
        TestCase {
            output: format!("output/v3.0/components/{dir}/{rs_name}").into(),
            schema: format!("examples/v3.0/src/components/{dir}/{yaml_name}").into(),
            example: format!("examples/v3.0/src/components/{dir}/{rs_name}").into(),
            module_name: yaml_name.replace(".yaml", ""),
            phantom: Default::default(),
        }
    }
}
