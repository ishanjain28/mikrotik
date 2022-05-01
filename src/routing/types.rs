use crate::serde_helpers::deserialize_bool;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    #[serde(rename = ".id")]
    pub id: String,
    #[serde(rename = ".nextid")]
    pub nextid: Option<String>,
    pub action: String,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub inactive: bool,
    #[serde(rename = "src-address")]
    pub src_address: String,
    pub table: String,
}
