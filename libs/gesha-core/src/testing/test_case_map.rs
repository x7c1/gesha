use crate::testing::TestCase;
use crate::{Error, Result};
use std::collections::HashMap;
use std::future::Future;
use tokio::task::{Id, JoinError, JoinHandle};
use tracing::Instrument;

#[derive(Default)]
pub struct TestCaseMap<A>(HashMap<Id, TestCase<A>>);

impl<A> TestCaseMap<A> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn push(mut self, id: Id, case: TestCase<A>) -> Self {
        self.0.insert(id, case);
        self
    }

    pub fn extract(&mut self, id: Id) -> Result<TestCase<A>> {
        self.0
            .remove(&id)
            .ok_or_else(|| Error::ThreadNotFound(id.to_string()))
    }

    pub fn accumulate<B>(
        (mut handles, map): (Vec<B>, Self),
        (id, case, handle): (Id, TestCase<A>, B),
    ) -> (Vec<B>, Self) {
        handles.push(handle);
        (handles, map.push(id, case))
    }

    pub fn flatten<B>(&mut self, result: std::result::Result<Result<B>, JoinError>) -> Result<B> {
        match result {
            Ok(x) => x,
            Err(cause) => Err(Error::JoinError {
                schema_path: self.extract(cause.id())?.schema,
                cause,
            }),
        }
    }
}

pub fn run_parallel<X, Y, F, Fut>(
    xs: Vec<TestCase<X>>,
    f: F,
) -> (Vec<JoinHandle<Result<Y>>>, TestCaseMap<X>)
where
    F: Fn(TestCase<X>) -> Fut,
    Fut: Future<Output = Result<Y>> + Send + 'static,
    X: Clone,
    Y: Send + 'static,
{
    xs.into_iter()
        .map(|x| {
            let cloned = x.clone();
            let handle = tokio::spawn(f(x).in_current_span());
            (handle.id(), cloned, handle)
        })
        .fold((vec![], TestCaseMap::new()), TestCaseMap::accumulate)
}
