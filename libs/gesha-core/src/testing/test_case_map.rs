use crate::testing::TestCase;
use crate::Error;
use std::collections::HashMap;
use tokio::task::{Id, JoinError};

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

    pub fn extract(&mut self, id: Id) -> crate::Result<TestCase<A>> {
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

    pub fn flatten<B>(&mut self, result: Result<crate::Result<B>, JoinError>) -> crate::Result<B> {
        match result {
            Ok(x) => x,
            Err(cause) => Err(Error::JoinError {
                schema_path: self.extract(cause.id())?.schema,
                cause,
            }),
        }
    }
}
