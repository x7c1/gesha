/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct SamplePet {
        pub id: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub foo: Option<sample_pet::Foo>,
    }

    pub mod sample_pet {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct Foo {
            pub registered_profile: foo::RegisteredProfile,
        }

        pub mod foo {
            use serde::Deserialize;
            use serde::Serialize;

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct RegisteredProfile {
                pub name: String,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub current_location: Option<registered_profile::CurrentLocation>,
            }

            pub mod registered_profile {
                use serde::Deserialize;
                use serde::Serialize;

                #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
                pub struct CurrentLocation {
                    pub latitude: f64,
                    pub longitude: f64,
                }
            }
        }
    }
}
