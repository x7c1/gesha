use crate::testing::Error::TaskNotFound;
use crate::testing::TestCase;
use crate::{Error, Result, testing};
use futures::future::join_all;
use std::collections::HashMap;
use std::future::Future;
use tokio::task::{Id, JoinError, JoinHandle};
use tracing::Instrument;

pub fn run_parallel<A, B, F, Fut>(cases: Vec<TestCase<A>>, f: F) -> Joiner<A, B>
where
    F: Fn(TestCase<A>) -> Fut,
    Fut: Future<Output = Result<B>> + Send + 'static,
    B: Send + 'static,
{
    let (handles, map) = cases
        .into_iter()
        .map(|case| {
            let cloned = case.clone();
            let handle = tokio::spawn(f(case).in_current_span());
            (handle.id(), cloned, handle)
        })
        .fold((vec![], TestCaseMap::new()), TestCaseMap::accumulate);

    Joiner { handles, map }
}

pub struct Joiner<A, B> {
    map: TestCaseMap<A>,
    handles: Vec<JoinHandle<Result<B>>>,
}

impl<A, B> Joiner<A, B> {
    pub async fn join_all<F, Y>(mut self, f: F) -> (Vec<Y>, Vec<Error>)
    where
        F: Fn(&mut Vec<Y>, &mut Vec<Error>, Result<B>),
    {
        let (outputs, errors) = join_all(self.handles)
            .await
            .into_iter()
            .map(|result| self.map.flatten(result))
            .fold((vec![], vec![]), |(mut outputs, mut errors), result| {
                f(&mut outputs, &mut errors, result);
                (outputs, errors)
            });

        (outputs, errors)
    }

    pub async fn collect_errors(self) -> Vec<Error> {
        let (_, errors) = self
            .join_all(|_: &mut Vec<()>, errors, result| match result {
                Ok(_) => {}
                Err(e) => errors.push(e),
            })
            .await;

        errors
    }
}

#[derive(Default)]
struct TestCaseMap<A>(HashMap<Id, TestCase<A>>);

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
            .ok_or_else(|| TaskNotFound { id: id.to_string() }.into())
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
            Err(cause) => Err(testing::Error::JoinError {
                schema_path: self.extract(cause.id())?.schema,
                cause,
            })?,
        }
    }
}
