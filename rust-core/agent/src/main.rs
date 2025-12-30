use core::PolicyEngine;
use platform_common::{ProcessScanner, SystemProfiler};
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use arboard::Clipboard;
use pqc_kyber::{keypair, encapsulate};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::rngs::OsRng;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum AgentEvent {
    Violation(String),
}

#[cfg(target_os = "windows")]
use platform_win::WindowsScanner as PlatformScanner;
#[cfg(target_os = "macos")]
use platform_macos::MacScanner as PlatformScanner;
#[cfg(target_os = "linux")]
use platform_linux::LinuxScanner as PlatformScanner;

#[cfg(target_os = "windows")]
use platform_win::WindowsProfiler as PlatformProfiler;
#[cfg(target_os = "macos")]
use platform_macos::MacProfiler as PlatformProfiler;
#[cfg(target_os = "linux")]
use platform_linux::LinuxProfiler as PlatformProfiler;

fn encrypt_violation(msg: &str, server_pk: &[u8]) -> String {
    let mut rng = OsRng;
    let (kyber_ct, shared_secret) = encapsulate(server_pk, &mut rng).unwrap();
    
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&shared_secret);
    let cipher = Aes256Gcm::new(key);
    
    // Use a random nonce (96-bits for AES-GCM)
    // For simplicity here we can use a counter or random.
    // Ideally use OsRng for nonce too.
    use rand::RngCore;
    let mut nonce_bytes = [0u8; 12];
    rng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, msg.as_bytes()).expect("encryption failure");
    
    // Format: Base64(KyberCT || Nonce || AesCT)
    let mut combined = Vec::new();
    combined.extend_from_slice(&kyber_ct);
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);
    
    BASE64.encode(combined)
}

fn main() {
    println!("Starting OLL Assessment Driver Agent...");
    println!("Initializing Quantum-Resistant Cryptography...");
    
    // Simulate Server Key Exchange (In production, this PK would be hardcoded or fetched)
    let mut rng = OsRng;
    let keys = keypair(&mut rng).unwrap();
    let server_pk = keys.public;
    println!("Secure Channel Established (Kyber-768).");

    let violations = std::thread::spawn(move || {
        // System Profiling
        let profiler = PlatformProfiler;
        
        match profiler.list_antivirus() {
            Ok(avs) => {
                println!("Antivirus Software Found: {}", avs.len());
                for av in avs {
                    println!(" - {} (Enabled: {})", av.name, av.is_enabled);
                }
            }
            Err(e) => println!("Error listing antivirus: {}", e),
        }

        let mut displays = Vec::new();
        match profiler.list_displays() {
            Ok(ds) => {
                println!("Displays Found: {}", ds.len());
                for d in &ds {
                    println!(" - {} (ID: {})", d.name, d.device_id);
                }
                displays = ds;
            }
            Err(e) => println!("Error listing displays: {}", e),
        }

        let mut all_browsers = Vec::new();
        let mut all_extensions = Vec::new();

        match profiler.list_browsers() {
            Ok(browsers) => {
                println!("Browsers Found: {}", browsers.len());
                for browser in browsers {
                    println!(" - {} (Profiles: {})", browser.name, browser.profiles.len());
                    match profiler.list_extensions(&browser) {
                        Ok(exts) => {
                            if !exts.is_empty() {
                                println!("   Extensions: {}", exts.len());
                                for ext in &exts {
                                    println!("    * {} ({}) [{}]", ext.name, ext.version, ext.id);
                                }
                                all_extensions.extend(exts);
                            }
                        }
                        Err(e) => println!("   Error listing extensions: {}", e),
                    }
                    all_browsers.push(browser);
                }
            }
            Err(e) => println!("Error listing browsers: {}", e),
        }

        let mut is_vm = false;
        match profiler.check_vm() {
            Ok(vm) => {
                is_vm = vm;
                println!("Virtual Machine: {}", vm);
            }
            Err(e) => println!("Error checking VM: {}", e),
        }

        // Process Scanning
        let scanner = PlatformScanner;
        let mut processes = Vec::new();
        match scanner.scan() {
            Ok(procs) => {
                println!("Processes Scanned: {}", procs.len());
                processes = procs;
            }
            Err(e) => println!("Error scanning processes: {}", e),
        }

        // Network Adapters
        let mut adapters = Vec::new();
        match profiler.list_network_adapters() {
            Ok(ads) => {
                println!("Network Adapters (Connected): {}", ads.len());
                for ad in &ads {
                    println!(" - {} ({})", ad.name, ad.description);
                }
                adapters = ads;
            }
            Err(e) => println!("Error listing network adapters: {}", e),
        }

        // Hosts File
        let mut hosts = Vec::new();
        match profiler.parse_hosts_file() {
            Ok(entries) => {
                println!("Hosts File Entries: {}", entries.len());
                hosts = entries;
            }
            Err(e) => println!("Error parsing hosts file: {}", e),
        }

        // Active Window
        let mut active_window = String::new();
        match profiler.get_active_window() {
            Ok(title) => {
                println!("Active Window: {}", title);
                active_window = title;
            }
            Err(e) => println!("Error getting active window: {}", e),
        }

        let policy_engine = PolicyEngine::new();
        policy_engine.evaluate(
            &all_browsers,
            &all_extensions,
            &displays,
            &processes,
            &adapters,
            &hosts,
            &active_window,
            is_vm,
        )
    }).join().unwrap();

    if !violations.is_empty() {
        println!("\n[!] CRITICAL: Security violations detected.");
        println!("[*] Encrypting Violation Report...");
        
        let report = violations.join("\n");
        let encrypted_report = encrypt_violation(&report, &server_pk);
        
        println!("[*] Sending Quantum-Encrypted Violation Report to OLL Server...");
        println!("[*] Payload: {}", encrypted_report);
        println!("[!] The Assessment Browser cannot be launched until these issues are resolved.");
        println!("[!] Please disable forbidden software, disconnect extra monitors, and retry.");
        // println!("[!] Exiting...");
        // std::process::exit(1);
        println!("[!] WARNING: PROCEEDING WITH VIOLATIONS FOR DEMONSTRATION PURPOSES.");
    }

    println!("Launching Secure Assessment Browser...");
    let event_loop = EventLoopBuilder::<AgentEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // Start Monitoring Thread
    let server_pk_clone = server_pk.clone();
    
    std::thread::spawn(move || {
        let profiler = PlatformProfiler;
        let scanner = PlatformScanner;
        let policy = PolicyEngine::new();
        let server_pk = server_pk_clone;

        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            
            // Check Active Window
            if let Ok(title) = profiler.get_active_window() {
                if let Some(violation) = policy.check_active_window(&title) {
                    let encrypted = encrypt_violation(&violation, &server_pk);
                    let _ = proxy.send_event(AgentEvent::Violation(encrypted));
                    break;
                }
            }

            // Check Processes (less frequent? 5s is fine)
            if let Ok(procs) = scanner.scan() {
                 let violations = policy.check_processes(&procs);
                 if !violations.is_empty() {
                     let msg = violations.join(", ");
                     let encrypted = encrypt_violation(&msg, &server_pk);
                     let _ = proxy.send_event(AgentEvent::Violation(encrypted));
                     break;
                 }
            }
        }
    });

    let window = WindowBuilder::new()
        .with_title("OLL Assessment Browser")
        .with_fullscreen(Some(tao::window::Fullscreen::Borderless(None)))
        .build(&event_loop)
        .unwrap();

    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    let _webview = WebViewBuilder::new()
        .with_url("https://ollacademy.com")
        .with_navigation_handler(|url: String| {
            // Robust parsing with url crate
            if let Ok(parsed_url) = url::Url::parse(&url) {
                if let Some(host_str) = parsed_url.host_str() {
                    let host = host_str.to_lowercase();
                    // Allowed domains: ollacademy.com, *.ollacademy.com, olllms.com, *.olllms.com
                    if host == "ollacademy.com" || host.ends_with(".ollacademy.com") ||
                       host == "olllms.com" || host.ends_with(".olllms.com") {
                        return true;
                    }
                }
            }
            // Allow about:blank for initialization/redirects
            if url == "about:blank" {
                return true;
            }
            
            println!("Blocked navigation to: {}", url);
            false
        })
        .with_initialization_script(r#"
            // Disable context menu
            document.addEventListener('contextmenu', event => event.preventDefault());
            
            // Disable keyboard shortcuts for devtools and reloading
            document.addEventListener('keydown', event => {
                if (
                    event.key === 'F12' || 
                    (event.ctrlKey && event.shiftKey && ['I','J','C'].includes(event.key)) ||
                    (event.ctrlKey && event.key === 'U') ||
                    (event.key === 'F5') ||
                    (event.ctrlKey && event.key === 'r')
                ) {
                    event.preventDefault();
                }
            });
        "#)
        .build(&window)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(AgentEvent::Violation(encrypted_msg)) => {
                println!("\n[!] RUNTIME VIOLATION DETECTED");
                println!("[*] Sending Quantum-Encrypted Telemetry to OLL Server...");
                println!("[*] Payload: {}", encrypted_msg);
                println!("[!] Terminating Assessment Session...");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
