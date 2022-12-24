/*
    Generated by gesha command; DO NOT EDIT BY HAND!
*/
pub mod schemas {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct SamplePet {
        pub id: i64,
        pub registered_profile: Option<sample_pet::RegisteredProfile>,
    }

    pub mod sample_pet {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
        pub struct RegisteredProfile {
            pub name: String,
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