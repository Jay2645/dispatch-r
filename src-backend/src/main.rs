#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{utils::config::AppUrl, WindowUrl};

fn main() {
  // Default webserver port is 12180
  let port = 12180;

  // This sets up Tauri to create a web server at the specified port
  let mut context = tauri::generate_context!();
  let url = format!("http://localhost:{}", port).parse().unwrap();
  let window_url = WindowUrl::External(url);
  // Rewrite the Tauri config so the IPC is enabled on this URL
  context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
  context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());

  // Display Tauri application
  tauri::Builder::default()
    .plugin(tauri_plugin_localhost::Builder::new(port).build())
    .run(context)
    .expect("error while running tauri application");
}
