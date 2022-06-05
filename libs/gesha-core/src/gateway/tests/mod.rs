use crate::conversions::{ToOpenApi, ToRustType};
use crate::gateway;
use crate::gateway::{detect_diff, Reader, Writer};
use crate::renderer::Renderer;
use crate::targets::rust_type::Modules;
use openapi_types::v3_0;
use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug)]
pub struct TestCase<A> {
    output: PathBuf,
    schema: PathBuf,
    expected: PathBuf,
    phantom: PhantomData<A>,
}

impl TestCase<(v3_0::ComponentsObject, Modules)> {
    pub fn from<A>(yaml_names: Vec<A>) -> Vec<Self>
    where
        A: Into<Cow<'static, str>>,
    {
        yaml_names
            .into_iter()
            .map(|x| x.into())
            .map(Self::create)
            .collect()
    }

    fn create(yaml_name: Cow<str>) -> Self {
        let rs_name = yaml_name.replace(".yaml", ".rs");
        TestCase {
            output: format!("output/v3.0/components/{rs_name}").into(),
            schema: format!("examples/v3.0/components/{yaml_name}").into(),
            expected: format!("examples/v3.0/components/{rs_name}").into(),
            phantom: Default::default(),
        }
    }
}

pub fn test_rust_type<A, B>(target: TestCase<(A, B)>) -> gateway::Result<()>
where
    A: Debug + ToOpenApi,
    B: Debug + ToRustType<A> + Renderer,
{
    println!("target> {:#?}", target);

    let reader = Reader::new::<A>();
    let rust_types: B = reader.open_rust_type(target.schema)?;
    println!("rust_types> {:#?}", rust_types);

    let writer = Writer {
        path: target.output.clone(),
        preamble: None,
    };
    writer.create_file(rust_types)?;
    detect_diff(target.output, target.expected)?;
    Ok(())
}
