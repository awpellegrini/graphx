#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;


#[path = "graphs/generate.rs"] mod generate;
mod test;
#[path = "graphs/dfs.rs"] mod dfs;

#[tauri::command]
fn testconnect() -> String {
 let res = test::testconnect();
  format!("{:?}", res)
}

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

#[tauri::command]
fn get_connected_subgraph(vertex: String) -> String {
 let res = dfs::get_connected_subgraph(vertex);
  format!("{:?}", res)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      testconnect,
      generate_graph_example,
      generate_graph_random,
      get_connected_subgraph])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  // db::create_db();
}