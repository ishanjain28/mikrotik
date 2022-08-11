use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Hash, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    #[serde(rename = ".id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}
