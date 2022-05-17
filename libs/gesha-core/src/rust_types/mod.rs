#[derive(Debug)]
pub struct ComponentsType {
    pub schemas: Vec<SchemaType>,
}

#[derive(Debug)]
pub struct SchemaType {
    pub name: String,
}
