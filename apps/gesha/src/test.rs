use gesha_core::gateway;
use gesha_core::gateway::{run_test, TestTarget};

pub fn run_tests() -> gateway::Result<()> {
    println!("run_tests>");
    run_test(TestTarget {
        schema: "examples/v3.0/components/pet.yaml".into(),
        output: "output/v3.0/components/pet.rs".into(),
        expected: "examples/v3.0/components/pet.rs".into(),
    })
}
