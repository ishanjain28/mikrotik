use crate::serde_helpers::{
    deserialize_bool, deserialize_u16, deserialize_u64, maybe_deserialize_u16,
};
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InterfaceType {
    #[serde(rename = "vlan")]
    VLAN,
    #[serde(rename = "ether")]
    Ethernet,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "wg")]
    Wireguard,
    #[serde(rename = "pppoe-out")]
    PPPoE,
}

impl Default for InterfaceType {
    fn default() -> Self {
        Self::Ethernet
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Mtu {
    #[serde(rename = "auto")]
    Auto,
    Value(u16),
}

impl Default for Mtu {
    fn default() -> Self {
        Self::Auto
    }
}

pub fn maybe_deserialize_mtu<'de, D>(deserializer: D) -> Result<Option<Mtu>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    if s == "auto" {
        return Ok(Some(Mtu::Auto));
    }

    match s.parse::<u16>() {
        Ok(v) => Ok(Some(Mtu::Value(v))),
        Err(_) => Err(Error::invalid_value(Unexpected::Str(s), &"an integer")),
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    #[serde(rename = ".id")]
    pub id: String,
    #[serde(default, rename = "actual-mtu", deserialize_with = "deserialize_u16")]
    pub actual_mtu: u16,
    pub comment: Option<String>,
    #[serde(rename = "default-name")]
    pub default_name: Option<String>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    #[serde(default, rename = "fp-rx-byte", deserialize_with = "deserialize_u64")]
    pub fp_rx_byte: u64,
    #[serde(default, rename = "fp-rx-packet", deserialize_with = "deserialize_u64")]
    pub fp_rx_packet: u64,
    #[serde(default, rename = "fp-tx-byte", deserialize_with = "deserialize_u64")]
    pub fp_tx_byte: u64,
    #[serde(default, rename = "fp-tx-packet", deserialize_with = "deserialize_u64")]
    pub fp_tx_packet: u64,
    #[serde(default, deserialize_with = "deserialize_u16")]
    pub l2mtu: u16,
    #[serde(rename = "last-link-up-time")]
    pub last_link_up_time: Option<String>,
    #[serde(default, rename = "link-downs", deserialize_with = "deserialize_u16")]
    pub link_downs: u16,
    #[serde(rename = "mac-address")]
    pub mac_address: Option<String>,
    #[serde(
        default,
        rename = "max-l2mtu",
        deserialize_with = "maybe_deserialize_u16"
    )]
    pub max_l2mtu: Option<u16>,
    #[serde(default, deserialize_with = "maybe_deserialize_mtu")]
    pub mtu: Option<Mtu>,
    pub name: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub running: bool,
    #[serde(rename = "rx-byte", deserialize_with = "deserialize_u64")]
    pub rx_byte: u64,
    #[serde(rename = "rx-drop", deserialize_with = "deserialize_u64")]
    pub rx_drop: u64,
    #[serde(rename = "rx-error", deserialize_with = "deserialize_u64")]
    pub rx_error: u64,
    #[serde(rename = "rx-packet", deserialize_with = "deserialize_u64")]
    pub rx_packet: u64,
    pub slave: Option<String>,
    #[serde(rename = "tx-byte", deserialize_with = "deserialize_u64")]
    pub tx_byte: u64,
    #[serde(rename = "tx-drop", deserialize_with = "deserialize_u64")]
    pub tx_drop: u64,
    #[serde(rename = "tx-error", deserialize_with = "deserialize_u64")]
    pub tx_error: u64,
    #[serde(rename = "tx-packet", deserialize_with = "deserialize_u64")]
    pub tx_packet: u64,
    #[serde(rename = "tx-queue-drop", deserialize_with = "deserialize_u64")]
    pub tx_queue_drop: u64,
    #[serde(rename = "type")]
    pub type_field: InterfaceType,
    #[serde(rename = "last-link-down-time")]
    pub last_link_down_time: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireguardInterface {
    #[serde(rename = ".id")]
    pub id: String,
    pub comment: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    #[serde(rename = "listen-port", deserialize_with = "deserialize_u16")]
    pub listen_port: u16,
    #[serde(deserialize_with = "deserialize_u16")]
    pub mtu: u16,
    pub name: String,
    #[serde(rename = "private-key")]
    pub private_key: String,
    #[serde(rename = "public-key")]
    pub public_key: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub running: bool,
}
