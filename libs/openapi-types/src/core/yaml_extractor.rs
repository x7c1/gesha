#[macro_export]
macro_rules! define_yaml_extractor {
    ($err: ident) => {
        use $crate::core::OutputOptionOps;
        use $crate::yaml::{YamlError, YamlMap, YamlValue};
        use $crate::{Error, Output};

        pub trait YamlExtractor<A>
        where
            A: TryFrom<YamlValue, Error = YamlError>,
        {
            // TODO: rename after extract_if_exists is removed
            fn extract_if_exists2(&mut self, key: &str) -> Output<Option<A>>;
        }

        impl<A> YamlExtractor<A> for YamlMap
        where
            A: TryFrom<YamlValue, Error = YamlError>,
        {
            fn extract_if_exists2(&mut self, key: &str) -> Output<Option<A>> {
                let result = self.remove_if_exists2::<A>(key);
                let result = result.map_err(to_crate_error);
                let maybe = result.maybe();
                maybe.bind_errors($crate::with_key(key))
            }
        }

        fn to_crate_error(e: YamlError) -> Error {
            match e {
                YamlError::TypeMismatch { found, expected } => {
                    Error::SpecViolation($crate::SpecViolation::from($err::TypeMismatch {
                        found,
                        expected,
                    }))
                }
                YamlError::UnknownType { found } => {
                    Error::Unsupported($crate::Unsupported::UnknownType { found })
                }
            }
        }
    };
}
