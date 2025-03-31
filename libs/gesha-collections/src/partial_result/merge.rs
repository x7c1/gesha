use crate::partial_result::PartialResult;

pub trait MergeOps<A, E> {
    fn merge(self) -> PartialResult<Vec<A>, E>;
}

impl<A, E> MergeOps<A, E> for Vec<PartialResult<A, E>> {
    fn merge(self) -> PartialResult<Vec<A>, E> {
        let init = (vec![], vec![]);
        let tuples = self.into_iter().fold(init, |(mut xs, mut ys), x| {
            let (a, mut errors) = x.into_tuple();
            xs.push(a);
            ys.append(&mut errors);
            (xs, ys)
        });
        PartialResult(tuples.0, tuples.1)
    }
}

impl<A, E> MergeOps<A, E> for Vec<Result<A, E>> {
    fn merge(self) -> PartialResult<Vec<A>, E> {
        let init = (vec![], vec![]);
        let tuples = self.into_iter().fold(init, |(mut xs, mut ys), x| {
            match x {
                Ok(a) => xs.push(a),
                Err(e) => ys.push(e),
            }
            (xs, ys)
        });
        PartialResult(tuples.0, tuples.1)
    }
}

impl<A, E> MergeOps<A, E> for Result<Vec<A>, E> {
    fn merge(self) -> PartialResult<Vec<A>, E> {
        match self {
            Ok(xs) => PartialResult(xs, vec![]),
            Err(e) => PartialResult(vec![], vec![e]),
        }
    }
}

impl<A, E> MergeOps<A, E> for PartialResult<PartialResult<Vec<A>, E>, E> {
    fn merge(self) -> PartialResult<Vec<A>, E> {
        let PartialResult(xs, mut errors1) = self;
        let PartialResult(ys, errors2) = xs;
        errors1.extend(errors2);
        PartialResult(ys, errors1)
    }
}

impl<A, E> MergeOps<A, E> for Option<PartialResult<Vec<A>, E>> {
    fn merge(self) -> PartialResult<Vec<A>, E> {
        self.unwrap_or_else(|| PartialResult(vec![], vec![]))
    }
}

impl<A, E> MergeOps<A, E> for Result<PartialResult<Vec<A>, E>, E> {
    fn merge(self) -> PartialResult<Vec<A>, E> {
        self.unwrap_or_else(|e| PartialResult(vec![], vec![e]))
    }
}
