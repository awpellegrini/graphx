#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "graphs/generate.rs"] mod generate;

#[tauri::command]
fn generate_graph_example() -> String {
  let res = generate::generate_graph_example();
  format!("{:?}", res)
}

#[tauri::command]
fn generate_graph_random(vertices: usize, edges: usize) -> String {
 let res = generate::generate_graph_random(vertices, edges);
  format!("{:?}", res)
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      generate_graph_example,
      generate_graph_random])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}