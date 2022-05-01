use crate::serde_helpers::{deserialize_bool, deserialize_u16};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhcpServer {
    #[serde(rename = ".id")]
    pub id: String,
    #[serde(rename = "address-pool")]
    pub address_pool: String,
    pub authoritative: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub dynamic: bool,
    pub interface: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invalid: bool,
    #[serde(rename = "lease-script")]
    pub lease_script: String,
    #[serde(rename = "lease-time")]
    pub lease_time: String,
    pub name: String,
    #[serde(rename = "use-radius")]
    pub use_radius: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lease {
    #[serde(rename = ".id")]
    pub id: String,
    pub address: String,
    #[serde(rename = "address-lists")]
    pub address_lists: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub blocked: bool,
    #[serde(rename = "client-id")]
    pub client_id: Option<String>,
    #[serde(rename = "dhcp-option")]
    pub dhcp_option: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    pub dynamic: String,
    #[serde(rename = "host-name")]
    pub host_name: Option<String>,
    #[serde(rename = "last-seen")]
    pub last_seen: String,
    #[serde(rename = "mac-address")]
    pub mac_address: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub radius: bool,
    pub server: String,
    pub status: String,
    #[serde(rename = "active-address")]
    pub active_address: Option<String>,
    #[serde(rename = "active-client-id")]
    pub active_client_id: Option<String>,
    #[serde(rename = "active-mac-address")]
    pub active_mac_address: Option<String>,
    #[serde(rename = "active-server")]
    pub active_server: Option<String>,
    #[serde(rename = "expires-after")]
    pub expires_after: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    #[serde(rename = ".id")]
    pub id: String,
    pub address: String,
    #[serde(rename = "caps-manager")]
    pub caps_manager: String,
    pub comment: String,
    #[serde(rename = "dhcp-option")]
    pub dhcp_option: String,
    #[serde(rename = "dns-server")]
    pub dns_server: String,
    pub domain: Option<String>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub dynamic: bool,
    pub gateway: String,
    pub netmask: String,
    #[serde(rename = "ntp-server")]
    pub ntp_server: String,
    #[serde(rename = "wins-server")]
    pub wins_server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(rename = ".id")]
    pub id: String,
    #[serde(rename = "actual-interface")]
    pub actual_interface: String,
    pub address: String,
    pub comment: Option<String>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub dynamic: bool,
    pub interface: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invalid: bool,
    pub network: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DhcpClient {
    #[serde(rename = ".id")]
    pub id: String,
    #[serde(rename = "add-default-route")]
    pub add_default_route: String,
    pub address: String,
    #[serde(rename = "dhcp-options")]
    pub dhcp_options: String,
    #[serde(rename = "dhcp-server")]
    pub dhcp_server: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub dynamic: bool,
    #[serde(rename = "expires-after")]
    pub expires_after: String,
    pub gateway: String,
    pub interface: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invalid: bool,
    #[serde(rename = "primary-dns")]
    pub primary_dns: String,
    #[serde(rename = "secondary-dns")]
    pub secondary_dns: String,
    pub status: String,
    #[serde(rename = "use-peer-dns", deserialize_with = "deserialize_bool")]
    pub use_peer_dns: bool,
    #[serde(rename = "use-peer-ntp", deserialize_with = "deserialize_bool")]
    pub use_peer_ntp: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    #[serde(rename = ".id")]
    pub id: String,
    pub comment: Option<String>,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    pub distance: String,
    #[serde(rename = "dst-address")]
    pub dst_address: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub dynamic: bool,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub ecmp: bool,
    pub gateway: String,
    #[serde(rename = "hw-offloaded", deserialize_with = "deserialize_bool")]
    pub hw_offloaded: bool,
    #[serde(rename = "immediate-gw")]
    pub immediate_gw: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub inactive: bool,
    #[serde(rename = "pref-src")]
    pub pref_src: Option<String>,
    #[serde(rename = "routing-table")]
    pub routing_table: String,
    #[serde(deserialize_with = "deserialize_u16")]
    pub scope: u16,
    #[serde(default, rename = "static", deserialize_with = "deserialize_bool")]
    pub static_field: bool,
    #[serde(rename = "suppress-hw-offload", deserialize_with = "deserialize_bool")]
    pub suppress_hw_offload: bool,
    #[serde(
        default,
        rename = "target-scope",
        deserialize_with = "deserialize_bool"
    )]
    pub target_scope: bool,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub active: bool,
    #[serde(default, deserialize_with = "deserialize_bool")]
    pub connect: bool,
    #[serde(rename = "local-address")]
    pub local_address: Option<String>,
}
