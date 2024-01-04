use serde::{Serialize, Deserialize};
use serde_yaml::Mapping;

/// Struct to represent the data we collect for a specific hsot
#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub os: String,         // TODO: make enum for this
    pub access: bool,
    pub domain: Option<String>
}

impl Host {
    /// Instantiate a new host struct
    pub fn new(hostname: Option<String>, os: Option<String>, access: Option<bool> , domain: Option<String>) -> Host{
        Host{
            hostname: hostname.unwrap_or(String::from("")),
            os: os.unwrap_or(String::from("")),
            access: access.unwrap_or(false),
            domain: domain
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostsFile {
    pub hosts: Mapping,
}