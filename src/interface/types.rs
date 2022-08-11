use crate::serde_helpers::{
    deserialize_bool, deserialize_u16, deserialize_u64, maybe_deserialize_u16,
};
use ipnetwork::IpNetwork;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::net::IpAddr;

#[derive(Debug, Hash, Eq, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "6to4-tunnel")]
    Tunnel6to4,
}

impl Default for InterfaceType {
    fn default() -> Self {
        Self::Ethernet
    }
}

#[derive(Debug, Hash, Eq, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Hash, Eq, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Hash, Eq, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

pub fn de_ip_addr_vector<'de, D>(deserializer: D) -> Result<AllowedAddresses, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    let mut answer = vec![];

    for addr in s.split(',') {
        match addr.parse::<IpNetwork>() {
            Ok(v) => answer.push(v),
            Err(_) => {
                return Err(Error::invalid_value(
                    Unexpected::Str(s),
                    &"an ip address with an optional mask",
                ))
            }
        }
    }

    Ok(AllowedAddresses(answer))
}

pub fn maybe_ip_addr<'de, D>(deserializer: D) -> Result<Option<IpAddr>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    if s == "" {
        return Ok(None);
    }

    match s.parse::<IpAddr>() {
        Ok(v) => Ok(Some(v)),
        Err(_) => Err(Error::invalid_value(Unexpected::Str(s), &"an ip address")),
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AllowedAddresses(pub Vec<IpNetwork>);

impl Serialize for AllowedAddresses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let output: Vec<String> = self.0.iter().map(|c| c.to_string()).collect();

        let output = output.join(",");

        serializer.serialize_str(&output)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireguardPeer {
    #[serde(rename = ".id")]
    pub id: String,
    #[serde(rename = "allowed-address", deserialize_with = "de_ip_addr_vector")]
    pub allowed_address: AllowedAddresses,
    #[serde(
        rename = "current-endpoint-address",
        deserialize_with = "maybe_ip_addr"
    )]
    pub current_endpoint_address: Option<IpAddr>,
    #[serde(rename = "current-endpoint-port", deserialize_with = "deserialize_u16")]
    pub current_endpoint_port: u16,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    #[serde(rename = "endpoint-address", deserialize_with = "maybe_ip_addr")]
    pub endpoint_address: Option<IpAddr>,
    #[serde(rename = "endpoint-port", deserialize_with = "deserialize_u16")]
    pub endpoint_port: u16,
    pub interface: String,
    #[serde(rename = "last-handshake")]
    pub last_handshake: Option<String>,
    #[serde(rename = "persistent-keepalive")]
    pub persistent_keepalive: Option<String>,
    #[serde(rename = "public-key")]
    pub public_key: String,
    #[serde(deserialize_with = "deserialize_u64")]
    pub rx: u64,
    #[serde(deserialize_with = "deserialize_u64")]
    pub tx: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddWireguardPeerInput {
    #[serde(rename = "allowed-address")]
    pub allowed_address: AllowedAddresses,
    pub comment: Option<String>,
    pub disabled: bool,
    #[serde(rename = "endpoint-address")]
    pub endpoint_address: Option<IpAddr>,
    #[serde(rename = "endpoint-port")]
    pub endpoint_port: u16,
    pub interface: String,
    #[serde(rename = "persistent-keepalive")]
    /// Time in seconds
    pub persistent_keepalive: Option<u64>,
    #[serde(rename = "preshared-key")]
    pub preshared_key: Option<String>,
    #[serde(rename = "public-key")]
    pub public_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddWireguardPeerOutput {
    pub ret: String,
}
