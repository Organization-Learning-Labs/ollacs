use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntivirusInfo {
    pub name: String,
    pub is_enabled: bool,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub name: String,
    pub version: String,
    pub id: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserInfo {
    pub name: String,
    pub version: Option<String>,
    pub install_path: String,
    pub profiles: Vec<String>, // Paths to profiles
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub name: String,
    pub device_id: String,
    pub is_primary: bool, // Note: Difficult to determine via simple WMI PnPEntity, but we can try or default to false
}

pub trait SystemProfiler {
    fn list_antivirus(&self) -> Result<Vec<AntivirusInfo>, String>;
    fn list_browsers(&self) -> Result<Vec<BrowserInfo>, String>;
    fn list_extensions(&self, browser: &BrowserInfo) -> Result<Vec<ExtensionInfo>, String>;
    fn list_displays(&self) -> Result<Vec<DisplayInfo>, String>;
    fn list_network_adapters(&self) -> Result<Vec<NetworkAdapterInfo>, String>;
    fn parse_hosts_file(&self) -> Result<Vec<HostsEntry>, String>;
    fn get_active_window(&self) -> Result<String, String>;
    fn check_vm(&self) -> Result<bool, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAdapterInfo {
    pub name: String,
    pub description: String,
    pub mac_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostsEntry {
    pub ip: String,
    pub domain: String,
}

pub trait ProcessScanner {
    fn scan(&self) -> Result<Vec<ProcessInfo>, String>;
}

pub trait Blocker {
    fn block(&self, pid: u32) -> bool;
}
