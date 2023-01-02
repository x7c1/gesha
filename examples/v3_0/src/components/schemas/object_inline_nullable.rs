pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct SamplePet {
        pub id: i64,
        pub registered_profile: Option<sample_pet::RegisteredProfile>,
    }

    pub mod sample_pet {
        use super::super::core::Patch;
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct RegisteredProfile {
            #[serde(default, skip_serializing_if = "Patch::is_absent")]
            pub name: Patch<String>,
        }
    }
}

pub mod core {
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Patch<A> {
        Absent,
        Null,
        Value(A),
    }

    impl<A> Patch<A> {
        pub fn is_absent(&self) -> bool {
            matches!(self, Patch::Absent)
        }
    }

    impl<T> Default for Patch<T> {
        fn default() -> Self {
            Patch::Absent
        }
    }

    impl<T> From<Option<T>> for Patch<T> {
        fn from(opt: Option<T>) -> Patch<T> {
            match opt {
                Some(v) => Patch::Value(v),
                None => Patch::Null,
            }
        }
    }

    impl<'de, A> Deserialize<'de> for Patch<A>
    where
        A: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Option::deserialize(deserializer).map(Patch::from)
        }
    }

    impl<T: Serialize> Serialize for Patch<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Patch::Null => serializer.serialize_none(),
                Patch::Value(v) => v.serialize(serializer),
                Patch::Absent => Err(serde::ser::Error::custom(format!(
                    "Maybe fields need to be annotated with: {}",
                    r#"#[serde(default, skip_serializing_if = "Patch::is_absent")]"#
                ))),
            }
        }
    }
}
