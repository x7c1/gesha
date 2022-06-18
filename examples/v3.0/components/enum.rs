pub mod schemas {
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum StringEnum1 {
        Error1,
        Error2,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum StringEnum2 {
        ErrorFoo,
        ErrorBar,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum StringEnum3 {
        Asc,
        Desc,
    }
}
