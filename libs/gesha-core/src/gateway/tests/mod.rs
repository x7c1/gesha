use crate::gateway;
use crate::gateway::{detect_diff, Reader, Writer};
use crate::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestTarget {
    output: PathBuf,
    schema: PathBuf,
    expected: PathBuf,
}

impl TestTarget {
    pub fn new(yaml_names: Vec<&str>) -> Vec<Self> {
        yaml_names.into_iter().map(to_target).collect()
    }
}

pub fn run_test(target: TestTarget) -> gateway::Result<()> {
    println!("target> {:#?}", target);

    let reader = Reader::new::<v3_0::ComponentsObject>();
    let rust_types: Modules = reader.open_rust_type(target.schema)?;
    println!("components> {:#?}", rust_types);

    let writer = Writer {
        path: target.output.clone(),
        preamble: None,
    };
    writer.create_file(rust_types)?;
    detect_diff(target.output, target.expected)?;
    Ok(())
}

fn to_target(yaml_name: &str) -> TestTarget {
    let rs_name = yaml_name.replace(".yaml", ".rs");
    TestTarget {
        output: format!("output/v3.0/components/{rs_name}").into(),
        schema: format!("examples/v3.0/components/{yaml_name}").into(),
        expected: format!("examples/v3.0/components/{rs_name}").into(),
    }
}
