#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json;
#[path = "db.rs"] mod db;
#[path = "graphs/examples.rs"] mod examples;
#[path = "graphs/objectification.rs"] mod objectification;
#[path = "graphs/dfs.rs"] mod dfs;


#[tauri::command]
fn playground_initialize() -> String {
  //TODO: intialize playground in the db
  println!("playground_initialize");

  let response = serde_json::json!({
    "graph": {
      "vertices": [],
      "edges": []
    },
    "adj_mat": []
  });

  return serde_json::to_string(&response).unwrap();
}

#[tauri::command]
fn generate_graph_example() -> String {
  let (example_graph, adj_mat) = examples::create_graph_example();

  db::clear_db();
  db::create_graph(&example_graph);

  let response = serde_json::json!({
    "graph": objectification::objectify(&example_graph),
    "adj_mat": adj_mat
  });

  return serde_json::to_string(&response).unwrap();
}

#[tauri::command]
fn generate_graph_random(vertices: usize, edges: usize, directed:bool) -> String {
  let (random_graph, adj_mat) = examples::create_graph_random(vertices, edges, directed);
  
  db::clear_db();
  db::create_graph(&random_graph);

  let response = serde_json::json!({
    "graph": objectification::objectify(&random_graph),
    "adj_mat": adj_mat
  });
  
  return serde_json::to_string(&response).unwrap();
}

#[tauri::command]
fn get_connected_subgraph(vertex: String) -> String {
  let v = db::get_vertices();
  let e = db::get_edges();

  let response = dfs::get_connected_subgraph(v, e, vertex);

  return serde_json::to_string(&response).unwrap()
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      playground_initialize,
      generate_graph_example,
      generate_graph_random,
      get_connected_subgraph,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}