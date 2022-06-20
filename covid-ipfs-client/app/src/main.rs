#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cors;

use tauri::{window::WindowBuilder, WindowUrl};

fn main() {
  let port = portpicker::pick_unused_port().expect("failed to find unused port");

  tauri::Builder::default()
    .plugin(cors::Builder::new(3001).build())
    .setup(move |app| {
      WindowBuilder::new(
        app,
        "main".to_string(),
        WindowUrl::External(format!("http://localhost:{}", 3001).parse().unwrap()),
      )
      .title("Covid IPFS")
      .fullscreen(false)
      .resizable(false)
      .inner_size(850.0, 700.0)
      .build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
