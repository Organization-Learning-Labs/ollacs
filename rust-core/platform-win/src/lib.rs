use platform_common::{
    AntivirusInfo, Blocker, BrowserInfo, DisplayInfo, ExtensionInfo, HostsEntry, NetworkAdapterInfo,
    ProcessInfo, ProcessScanner, SystemProfiler,
};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use walkdir::WalkDir;
use winreg::enums::*;
use winreg::RegKey;
use wmi::{COMLibrary, WMIConnection};

pub struct WindowsProfiler;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32NetworkAdapter {
    name: String,
    description: Option<String>,
    m_a_c_address: Option<String>,
    net_connection_status: Option<u16>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32Process {
    process_id: u32,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ComputerSystem {
    manufacturer: Option<String>,
    model: Option<String>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct AntiVirusProduct {
    display_name: String,
    product_state: u32, // Bitmask
    // instance_guid: String,
    // path_to_signed_product_exe: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct PnPDevice {
    name: Option<String>,
    device_i_d: Option<String>,
    service: Option<String>,
    status: Option<String>,
}

impl SystemProfiler for WindowsProfiler {
    fn list_antivirus(&self) -> Result<Vec<AntivirusInfo>, String> {
        let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
        let wmi_con = WMIConnection::with_namespace_path("ROOT\\SecurityCenter2", com_con)
            .map_err(|e| e.to_string())?;

        let results: Vec<AntiVirusProduct> = wmi_con
            .raw_query("SELECT * FROM AntivirusProduct")
            .map_err(|e| e.to_string())?;

        let mut av_infos = Vec::new();
        for av in results {
            // ProductState encoding:
            // 0x1000 (4096) = Enabled (bit 12)
            // 0x0010 (16)   = Updated (bit 4)
            // This is a simplification.
            let is_enabled = (av.product_state & 0x1000) != 0;
            av_infos.push(AntivirusInfo {
                name: av.display_name,
                is_enabled,
                version: None, // Version not easily available in WMI Standard
            });
        }
        Ok(av_infos)
    }

    fn list_browsers(&self) -> Result<Vec<BrowserInfo>, String> {
        let mut browsers = Vec::new();
        let mut seen_names = std::collections::HashSet::new();
        let hives = [HKEY_LOCAL_MACHINE, HKEY_CURRENT_USER];

        for hive in hives {
            let root = RegKey::predef(hive);
            if let Ok(clients) = root.open_subkey("SOFTWARE\\Clients\\StartMenuInternet") {
                for name in clients.enum_keys().filter_map(|x| x.ok()) {
                    let subkey = match clients.open_subkey(&name) {
                        Ok(k) => k,
                        Err(_) => continue,
                    };
                    let display_name: String = subkey.get_value("").unwrap_or(name.clone());
                    
                    if seen_names.contains(&display_name) {
                        continue;
                    }

                    // Get Install Path (shell/open/command)
                    let cmd_key = subkey.open_subkey("shell\\open\\command");
                    let install_path = if let Ok(k) = cmd_key {
                        let cmd: String = k.get_value("").unwrap_or_default();
                        cmd.replace("\"", "") // Simple cleanup
                    } else {
                        String::new()
                    };

                    // Detect Profiles (for Chrome/Edge)
                    let mut profiles = Vec::new();
                    let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
                    let mut user_data_path = PathBuf::new();
                    
                    if name.to_lowercase().contains("chrome") {
                         user_data_path.push(&local_app_data);
                         user_data_path.push("Google\\Chrome\\User Data");
                    } else if name.to_lowercase().contains("edge") {
                         user_data_path.push(&local_app_data);
                         user_data_path.push("Microsoft\\Edge\\User Data");
                    }

                    if user_data_path.exists() {
                         profiles.push(user_data_path.to_string_lossy().to_string());
                         // Could scan for "Profile *" folders here too
                    }

                    browsers.push(BrowserInfo {
                        name: display_name.clone(),
                        version: None,
                        install_path,
                        profiles,
                    });
                    seen_names.insert(display_name);
                }
            }
        }
        Ok(browsers)
    }

    fn list_extensions(&self, browser: &BrowserInfo) -> Result<Vec<ExtensionInfo>, String> {
        let mut extensions = Vec::new();
        for profile_path in &browser.profiles {
            let extensions_dir = PathBuf::from(profile_path).join("Default\\Extensions");
            if !extensions_dir.exists() {
                continue;
            }

            for entry in WalkDir::new(extensions_dir).min_depth(1).max_depth(1) {
                let entry = entry.map_err(|e| e.to_string())?;
                if !entry.file_type().is_dir() { continue; }
                
                let id = entry.file_name().to_string_lossy().to_string();
                
                // Find version dir inside
                for ver_entry in WalkDir::new(entry.path()).min_depth(1).max_depth(1) {
                     let ver_entry = ver_entry.map_err(|e| e.to_string())?;
                     if !ver_entry.file_type().is_dir() { continue; }
                     let version = ver_entry.file_name().to_string_lossy().to_string();
                     
                     // Read manifest.json for Name
                     let manifest_path = ver_entry.path().join("manifest.json");
                     if manifest_path.exists() {
                         if let Ok(file) = std::fs::File::open(manifest_path) {
                             let json: serde_json::Value = serde_json::from_reader(file).unwrap_or_default();
                             let name = json["name"].as_str().unwrap_or(&id).to_string();
                             
                             extensions.push(ExtensionInfo {
                                 name,
                                 version,
                                 id: id.clone(),
                                 is_enabled: true, // Hard to determine without parsing Preferences
                             });
                         }
                     }
                }
            }
        }
        Ok(extensions)
    }
    fn list_displays(&self) -> Result<Vec<DisplayInfo>, String> {
        let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
        let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMV2", com_con)
            .map_err(|e| e.to_string())?;

        // Query Win32_PnPEntity where Service = 'monitor'
        let results: Vec<PnPDevice> = wmi_con
            .raw_query("SELECT Name, DeviceID, Service, Status FROM Win32_PnPEntity WHERE Service = 'monitor'")
            .map_err(|e| e.to_string())?;

        let mut displays = Vec::new();
        for device in results {
            if let Some(status) = &device.status {
                if status != "OK" { continue; }
            }

            displays.push(DisplayInfo {
                name: device.name.unwrap_or_else(|| "Unknown Display".to_string()),
                device_id: device.device_i_d.unwrap_or_default(),
                is_primary: false, // Hard to determine primary via simple PnPEntity check
            });
        }
        Ok(displays)
    }

    fn check_vm(&self) -> Result<bool, String> {
        let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
        let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMV2", com_con)
            .map_err(|e| e.to_string())?;
        
        let results: Vec<ComputerSystem> = wmi_con
            .raw_query("SELECT Manufacturer, Model FROM Win32_ComputerSystem")
            .map_err(|e| e.to_string())?;

        if let Some(system) = results.first() {
            let manufacturer = system.manufacturer.as_deref().unwrap_or("").to_lowercase();
            let model = system.model.as_deref().unwrap_or("").to_lowercase();
            
            let vm_keywords = ["vmware", "virtualbox", "vbox", "qemu", "kvm", "microsoft corporation", "bochs", "parallels"];
            
            // Note: "microsoft corporation" can be physical (Surface) or virtual (Hyper-V). 
            // Better to look for "virtual" in model for generic check.
            for keyword in vm_keywords {
                if manufacturer.contains(keyword) || model.contains(keyword) {
                    if keyword == "microsoft corporation" && !model.contains("virtual") {
                         continue; 
                    }
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn list_network_adapters(&self) -> Result<Vec<NetworkAdapterInfo>, String> {
        let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
        let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMV2", com_con)
            .map_err(|e| e.to_string())?;

        // NetConnectionStatus = 2 (Connected)
        let results: Vec<Win32NetworkAdapter> = wmi_con
            .raw_query("SELECT Name, Description, MACAddress, NetConnectionStatus FROM Win32_NetworkAdapter WHERE NetConnectionStatus = 2")
            .map_err(|e| e.to_string())?;

        let mut adapters = Vec::new();
        for adapter in results {
            adapters.push(NetworkAdapterInfo {
                name: adapter.name,
                description: adapter.description.unwrap_or_default(),
                mac_address: adapter.m_a_c_address.unwrap_or_default(),
            });
        }
        Ok(adapters)
    }

    fn parse_hosts_file(&self) -> Result<Vec<HostsEntry>, String> {
        let hosts_path = PathBuf::from("C:\\Windows\\System32\\drivers\\etc\\hosts");
        if !hosts_path.exists() {
            return Ok(vec![]);
        }

        let file = File::open(hosts_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                entries.push(HostsEntry {
                    ip: parts[0].to_string(),
                    domain: parts[1].to_string(),
                });
            }
        }
        Ok(entries)
    }

    fn get_active_window(&self) -> Result<String, String> {
        use windows::{
            Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
            Win32::Foundation::HWND,
        };

        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            if hwnd.0 == std::ptr::null_mut() {
                return Ok("".to_string());
            }

            let mut buffer = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut buffer);
            if len > 0 {
                let title = String::from_utf16_lossy(&buffer[..len as usize]);
                Ok(title)
            } else {
                Ok("".to_string())
            }
        }
    }
}

pub struct WindowsScanner;
impl ProcessScanner for WindowsScanner {
    fn scan(&self) -> Result<Vec<ProcessInfo>, String> {
        let com_con = COMLibrary::new().map_err(|e| e.to_string())?;
        let wmi_con = WMIConnection::with_namespace_path("ROOT\\CIMV2", com_con)
            .map_err(|e| e.to_string())?;

        let results: Vec<Win32Process> = wmi_con
            .raw_query("SELECT ProcessId, Name FROM Win32_Process")
            .map_err(|e| e.to_string())?;

        Ok(results.into_iter().map(|p| ProcessInfo {
            pid: p.process_id,
            name: p.name,
        }).collect())
    }
}

pub struct WindowsBlocker;
impl Blocker for WindowsBlocker {
    fn block(&self, pid: u32) -> bool {
        println!("Blocking process {} on Windows", pid);
        true
    }
}
