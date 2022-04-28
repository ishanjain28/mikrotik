use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    #[serde(rename = ".id")]
    pub id: String,
    #[serde(rename = "actual-mtu")]
    pub actual_mtu: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "default-name")]
    pub default_name: Option<String>,
    pub disabled: String,
    #[serde(rename = "fp-rx-byte")]
    pub fp_rx_byte: String,
    #[serde(rename = "fp-rx-packet")]
    pub fp_rx_packet: String,
    #[serde(rename = "fp-tx-byte")]
    pub fp_tx_byte: String,
    #[serde(rename = "fp-tx-packet")]
    pub fp_tx_packet: String,
    pub l2mtu: Option<String>,
    #[serde(rename = "last-link-up-time")]
    pub last_link_up_time: Option<String>,
    #[serde(rename = "link-downs")]
    pub link_downs: String,
    #[serde(rename = "mac-address")]
    pub mac_address: Option<String>,
    #[serde(rename = "max-l2mtu")]
    pub max_l2mtu: Option<String>,
    pub mtu: Option<String>,
    pub name: String,
    pub running: String,
    #[serde(rename = "rx-byte")]
    pub rx_byte: String,
    #[serde(rename = "rx-drop")]
    pub rx_drop: String,
    #[serde(rename = "rx-error")]
    pub rx_error: String,
    #[serde(rename = "rx-packet")]
    pub rx_packet: String,
    pub slave: Option<String>,
    #[serde(rename = "tx-byte")]
    pub tx_byte: String,
    #[serde(rename = "tx-drop")]
    pub tx_drop: String,
    #[serde(rename = "tx-error")]
    pub tx_error: String,
    #[serde(rename = "tx-packet")]
    pub tx_packet: String,
    #[serde(rename = "tx-queue-drop")]
    pub tx_queue_drop: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "last-link-down-time")]
    pub last_link_down_time: Option<String>,
}
