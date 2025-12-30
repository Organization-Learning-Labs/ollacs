use platform_common::{
    AntivirusInfo, Blocker, BrowserInfo, DisplayInfo, ExtensionInfo, HostsEntry, NetworkAdapterInfo,
    ProcessInfo, ProcessScanner, SystemProfiler,
};

pub struct MacProfiler;
impl SystemProfiler for MacProfiler {
    fn list_antivirus(&self) -> Result<Vec<AntivirusInfo>, String> {
        Ok(vec![])
    }
    fn list_browsers(&self) -> Result<Vec<BrowserInfo>, String> {
        Ok(vec![])
    }
    fn list_extensions(&self, _browser: &BrowserInfo) -> Result<Vec<ExtensionInfo>, String> {
        Ok(vec![])
    }
    fn list_displays(&self) -> Result<Vec<DisplayInfo>, String> {
        Ok(vec![])
    }
    fn check_vm(&self) -> Result<bool, String> {
        Ok(false)
    }
    fn list_network_adapters(&self) -> Result<Vec<NetworkAdapterInfo>, String> {
        Ok(vec![])
    }
    fn parse_hosts_file(&self) -> Result<Vec<HostsEntry>, String> {
        Ok(vec![])
    }
    fn get_active_window(&self) -> Result<String, String> {
        Ok("Mac Window".to_string())
    }
}

pub struct MacScanner;
impl ProcessScanner for MacScanner {
    fn scan(&self) -> Result<Vec<ProcessInfo>, String> {
        Ok(vec![])
    }
}

pub struct MacBlocker;
impl Blocker for MacBlocker {
    fn block(&self, pid: u32) -> bool {
        println!("Blocking process {} on macOS", pid);
        true
    }
}
