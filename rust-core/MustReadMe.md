# OLL Assessment Driver - Complete Documentation

## Overview
The **OLL Assessment Driver** is a secure, cross-platform agent designed to provide a controlled environment for high-stakes assessments. It ensures integrity by locking down the endpoint, monitoring for cheating attempts, and communicating telemetry via a quantum-resistant secure channel.

## Key Features

### 1. Secure Browser Environment
*   **Embedded Browser**: Uses `wry` (WebView2 on Windows, WebKit on macOS/Linux) to launch a borderless, full-screen assessment interface.
*   **Navigation Locking**: Strictly restricts navigation to `*.ollacademy.com` and `*.olllms.com`. All other URLs are blocked via a robust URL parsing handler.
*   **Interface Hardening**:
    *   **Context Menu Disabled**: Right-clicking is blocked to prevent "Inspect Element" or saving assets.
    *   **DevTools Blocked**: Keyboard shortcuts (F12, Ctrl+Shift+I, etc.) are intercepted and disabled.
    *   **Reload/Print Blocked**: F5, Ctrl+R, Ctrl+P are disabled.
    *   **Clipboard Protection**: The clipboard is automatically cleared continuously to prevent copy-pasting content.

### 2. Comprehensive Anti-Cheat System
The agent performs rigorous checks before and during the session:
*   **Environment Scanning**: Detects Virtual Machines (VMware, VirtualBox, QEMU) to prevent sandboxed cheating.
*   **Process Blacklisting**: Blocks remote desktop tools (AnyDesk, TeamViewer), communication apps (Discord, Slack), and hacking tools (Wireshark, CheatEngine).
*   **Browser Extension Analysis**: Scans local Chrome/Edge profiles for suspicious extensions (e.g., "ChatGPT", "Postman", "Wappalyzer").
*   **Network & System**: Detects VPN/Proxy adapters and tampered `hosts` file entries (e.g., redirects for cheating sites).
*   **Hardware**: Detects multiple monitor setups to ensure the user is focused on a single screen.

### 3. Continuous Runtime Monitoring
*   A background thread runs every **5 seconds**.
*   **Active Window Check**: Ensures the assessment window remains in focus.
*   **Process Watchdog**: Terminates the session immediately if a forbidden tool is launched during the exam.

### 4. Quantum-Resistant Telemetry
*   **Hybrid Encryption Scheme**:
    *   **Key Exchange**: Uses **Kyber-768** (Post-Quantum Key Encapsulation Mechanism) to establish a secure shared secret.
    *   **Data Encryption**: Uses **AES-256-GCM** with the shared secret to encrypt payload data.
*   **Secure Reporting**: All security violations (initial scan results and runtime detections) are encrypted locally before being displayed or transmitted. This prevents attackers from easily analyzing the anti-cheat triggers.

## Architecture
The project is organized as a Rust Workspace:
*   **`agent`**: The main executable binary. Handles the WebView UI, Event Loop, Thread Management, and High-Level Logic.
*   **`core`**: Contains the `PolicyEngine` logic, blacklists, and detection rules.
*   **`platform-common`**: Defines traits (`SystemProfiler`, `ProcessScanner`) and shared data structures.
*   **`platform-win` / `platform-macos` / `platform-linux`**: OS-specific implementations using low-level APIs (WMI, WinAPI, etc.) for deep system inspection.

## Build Instructions

### Prerequisites
*   **Rust**: Version 1.92.0 or newer.
*   **WiX Toolset** (Windows only): Required for generating MSI installers.

### Development Build
To run the agent in development mode:
```bash
cargo run -p agent
```

### Production Release
To compile the optimized binary:
```bash
cargo build --release -p agent
```

## Creating Installers

### Windows (MSI)
1.  Install the **WiX Toolset** (v3 or v4) and add it to your PATH.
2.  Install `cargo-wix`:
    ```bash
    cargo install cargo-wix
    ```
3.  Generate the Installer:
    ```bash
    cargo wix --package agent
    ```
4.  Output: `target/wix/OLL Assessment Driver-0.1.0-x86_64-pc-windows-msvc.msi`

### macOS (DMG / App Bundle)
1.  Install `cargo-bundle`:
    ```bash
    cargo install cargo-bundle
    ```
2.  Generate Bundle:
    ```bash
    cargo bundle --release
    ```
3.  Output: `target/release/bundle/osx/`

### Linux (Debian/Ubuntu)
1.  Install `cargo-deb`:
    ```bash
    cargo install cargo-deb
    ```
2.  Generate Package:
    ```bash
    cargo deb -p agent
    ```
3.  Output: `target/debian/`

## Usage Guide
1.  **Launch**: Run the `agent` executable (or install via MSI).
2.  **Initialization**:
    *   The agent initializes the Quantum Crypto context.
    *   A secure channel is established (simulated).
3.  **Pre-Flight Check**:
    *   The system scans for violations.
    *   **If Clean**: The Secure Browser launches immediately.
    *   **If Violations Found**:
        *   A detailed report is generated.
        *   The report is **Quantum-Encrypted**.
        *   The session is blocked (Exit Code 1).
        *   *(Note: The current build has a demonstration bypass enabled to allow testing even with violations).*
4.  **Active Session**:
    *   Navigate only to allowed OLL Academy domains.
    *   Do not attempt to open other tools.
    *   Do not plug in extra monitors.
5.  **Termination**: The app exits automatically if a violation is detected or the user closes the window.

## Configuration
*   **Allowed Domains**: Hardcoded in `agent/src/main.rs` for security.
*   **Detection Rules**: Defined in `core/src/lib.rs`. Update the `suspicious_extension_keywords` or `forbidden_process_keywords` arrays to modify detection logic.

---
*Copyright © 2025 OLL Academy. All rights reserved.*
