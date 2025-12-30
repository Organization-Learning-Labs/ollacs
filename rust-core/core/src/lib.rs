
use platform_common::{BrowserInfo, DisplayInfo, ExtensionInfo, HostsEntry, NetworkAdapterInfo, ProcessInfo};

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn check_active_window(&self, active_window: &str) -> Option<String> {
        let active_window_lower = active_window.to_lowercase();
        let forbidden_window_keywords = ["chatgpt", "discord", "search", "google", "stack overflow", "copilot"];
        for keyword in forbidden_window_keywords {
             if active_window_lower.contains(keyword) {
                 return Some(format!("Forbidden Active Window detected: {}", active_window));
             }
        }
        None
    }

    pub fn check_processes(&self, processes: &[ProcessInfo]) -> Vec<String> {
        let mut violations = Vec::new();
        let forbidden_process_keywords = [
            "wireshark", "fiddler", "charles", "cheatengine", "obs", "xsplit", 
            "teamviewer", "anydesk", "discord", "skype", "slack", "telegram"
        ];
        
        for process in processes {
            let name_lower = process.name.to_lowercase();
            for keyword in forbidden_process_keywords {
                if name_lower.contains(keyword) {
                    violations.push(format!("Forbidden Process detected: {} (PID: {})", process.name, process.pid));
                }
            }
        }
        violations
    }
    pub fn evaluate(
        &self,
        browsers: &[BrowserInfo],
        extensions: &[ExtensionInfo],
        displays: &[DisplayInfo],
        processes: &[ProcessInfo],
        adapters: &[NetworkAdapterInfo],
        hosts: &[HostsEntry],
        active_window: &str,
        is_vm: bool,
    ) -> Vec<String> {
        println!("Evaluating policy...");
        
        // AI-enhanced detection list
        let suspicious_browser_keywords = ["tor", "comet", "ulaa"];
        let suspicious_extension_keywords = [
            "postman", "vulners", "shodan", "wappalyzer", 
            "gpt", "copilot", "perplexity", "ai assistant", 
            "hack", "proxy", "vpn", "requestly"
        ];
        let forbidden_process_keywords = [
            "wireshark", "fiddler", "charles", "cheatengine", "obs", "xsplit", 
            "teamviewer", "anydesk", "discord", "skype", "slack", "telegram"
        ];

        let mut violations = Vec::new();

        if is_vm {
            violations.push("Virtual Machine Detected".to_string());
        }

        for process in processes {
            let name_lower = process.name.to_lowercase();
            for keyword in forbidden_process_keywords {
                if name_lower.contains(keyword) {
                    violations.push(format!("Forbidden Process detected: {} (PID: {})", process.name, process.pid));
                }
            }
        }

        // Network Adapter Checks (VPN/Proxy)
        let vpn_keywords = ["tap", "tun", "vpn", "wireguard", "hamachi", "openvpn", "zerotier"];
        for adapter in adapters {
            let name_lower = adapter.name.to_lowercase();
            let desc_lower = adapter.description.to_lowercase();
            for keyword in vpn_keywords {
                if name_lower.contains(keyword) || desc_lower.contains(keyword) {
                    violations.push(format!("Suspicious Network Adapter detected: {} ({})", adapter.name, adapter.description));
                }
            }
        }

        // Hosts File Checks
        let suspicious_domains = ["cheat", "brainly", "quizlet", "coursehero", "chegg", "openai"];
        for entry in hosts {
            let domain_lower = entry.domain.to_lowercase();
            for keyword in suspicious_domains {
                if domain_lower.contains(keyword) {
                    violations.push(format!("Suspicious Hosts Entry: {} -> {}", entry.ip, entry.domain));
                }
            }
        }

        // Active Window Check
        // If the window title suggests they are not in the assessment (which has a specific title potentially)
        // or if it matches a forbidden tool
        let active_window_lower = active_window.to_lowercase();
        let forbidden_window_keywords = ["chatgpt", "discord", "search", "google", "stack overflow", "copilot"];
        for keyword in forbidden_window_keywords {
             if active_window_lower.contains(keyword) {
                 violations.push(format!("Forbidden Active Window detected: {}", active_window));
             }
        }

        for browser in browsers {
            let name_lower = browser.name.to_lowercase();
            for keyword in suspicious_browser_keywords {
                if name_lower.contains(keyword) {
                    violations.push(format!("Suspicious Browser detected: {}", browser.name));
                }
            }
        }

        for ext in extensions {
            let name_lower = ext.name.to_lowercase();
            for keyword in suspicious_extension_keywords {
                if name_lower.contains(keyword) {
                    violations.push(format!("Suspicious Extension detected: {} ({})", ext.name, ext.id));
                }
            }
        }

        if displays.len() > 1 {
            violations.push(format!("Multiple Displays detected: {} monitors found", displays.len()));
        }

        violations
    }
}
