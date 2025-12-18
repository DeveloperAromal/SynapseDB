pub enum Query {
    Switch(SwitchSchema),
}
#[derive(Debug, PartialEq, Clone)]
pub struct SwitchSchema{
    pub schema_name: String
}