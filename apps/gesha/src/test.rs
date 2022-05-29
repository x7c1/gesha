use gesha_core::gateway;
use gesha_core::gateway::{run_test, TestTarget};

pub fn run_tests() -> gateway::Result<()> {
    let yaml_names = vec!["pet.yaml"];

    TestTarget::new(yaml_names)
        .into_iter()
        .try_for_each(run_test)
}
