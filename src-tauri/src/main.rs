#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json;
#[path = "graphs/examples.rs"] mod examples;
#[path = "graphs/objectification.rs"] mod objectification;

#[tauri::command]
fn generate_graph_example() -> String {
  let (example_graph, adj_mat) = examples::generate_graph_example();

  let response = serde_json::json!({
    "graph": objectification::objectification(&example_graph),
    "adj_mat": adj_mat
  });

  return serde_json::to_string(&response).unwrap();
}

#[tauri::command]
fn generate_graph_random(vertices: usize, edges: usize) -> String {
  let (random_graph, adj_mat) = examples::generate_graph_random(vertices, edges);
  
  let response = serde_json::json!({
    "graph": objectification::objectification(&random_graph),
    "adj_mat": adj_mat
  });

  return serde_json::to_string(&response).unwrap();
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      generate_graph_example,
      generate_graph_random,
      // commands::get_connected_subgraph
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}