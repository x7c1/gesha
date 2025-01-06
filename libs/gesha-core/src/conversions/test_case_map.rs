use crate::conversions::TestCase;
use crate::Error;
use std::collections::HashMap;
use tokio::task::Id;

#[derive(Default)]
pub struct TestCaseMap<From, To>(HashMap<Id, TestCase<From, To>>);

impl<From, To> TestCaseMap<From, To> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn push(mut self, id: Id, case: TestCase<From, To>) -> Self {
        self.0.insert(id, case);
        self
    }
    pub fn extract(&mut self, id: Id) -> crate::Result<TestCase<From, To>> {
        self.0
            .remove(&id)
            .ok_or_else(|| Error::ThreadNotFound(id.to_string()))
    }

    pub fn accumulate<A>(
        (mut handles, map): (Vec<A>, Self),
        (id, case, handle): (Id, TestCase<From, To>, A),
    ) -> (Vec<A>, Self) {
        handles.push(handle);
        (handles, map.push(id, case))
    }
}
