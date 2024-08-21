#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tests;
mod db;

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn test_setup() {
  tests::test_setup();
}

#[tauri::command]
fn test_db() {
  println!("js invoked db test");
  db::test_db();
}



fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      greet,
      test_setup,
      test_db])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}