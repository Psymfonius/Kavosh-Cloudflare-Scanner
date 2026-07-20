// Prevents additional console window on Windows in release mode. DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

// =====================================================================
// Native Window Control Commands
// =====================================================================

#[tauri::command]
fn minimize_app(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn close_app(window: tauri::Window) {
    let _ = window.close();
}

#[tauri::command]
fn drag_app(window: tauri::Window) {
    let _ = window.start_dragging();
}

// =====================================================================
// Network Scanning Logic
// =====================================================================

#[derive(Serialize)]
pub struct PingResult {
    ip_port: String,
    ping: u128,
}

// Added 'async' keyword to run this command on a background thread pool,
// preventing the UI from freezing during concurrent network requests.
#[tauri::command]
async fn test_ip_ports(ip: String, max_timeout: u64) -> Vec<PingResult> {
    let ports = [443, 2053, 2083, 2087, 2096, 8443];
    let mut results = Vec::new();
    let timeout = Duration::from_millis(max_timeout);

    for port in ports {
        let ip_port = format!("{}:{}", ip, port);

        if let Ok(addr) = ip_port.parse::<SocketAddr>() {
            let start = Instant::now();

            if let Ok(_) = TcpStream::connect_timeout(&addr, timeout) {
                let ping = start.elapsed().as_millis();

                if ping <= max_timeout as u128 {
                    results.push(PingResult { ip_port, ping });
                }
            }
        }
    }

    results
}

// =====================================================================
// Main Application Entry Point
// =====================================================================

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            test_ip_ports,
            minimize_app,
            close_app,
            drag_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
