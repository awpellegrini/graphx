use indradb::{Database, MemoryDatastore, Vertex};
use rand::Rng; // 0.8
use serde::Serialize;


#[path = "../db.rs"] mod db;
use db::{add_vertex, add_edge};

#[path = "formatting.rs"] mod formatting;
use formatting::{GraphxGraph, GraphxVertex};

#[path = "types.rs"] mod types;

#[derive(Serialize)]
struct GraphData {
        graph: GraphxGraph,
        adj_mat: Vec<Vec<usize>>
}

pub fn generate_graph(db: &Database<MemoryDatastore>, adj_mat: Vec<Vec<usize>>) -> String {
    //VERTICES
    let vertices: Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery).unwrap();
    let formatted_vertices: Vec<GraphxVertex> = formatting::format_vertices_from_db(&vertices);
 
    //EDGES
    let edges: Vec<indradb::QueryOutputValue> = db.get(indradb::AllEdgeQuery).unwrap();
    let formatted_edges = formatting::format_edges_from_db(edges);

    //formatting for submit
    let graph = GraphxGraph {
        vertices: formatted_vertices,
        edges: formatted_edges
    };

    let graph_data = GraphData {
        graph: graph,
        adj_mat: adj_mat
    };

    return serde_json::to_string(&graph_data).unwrap();
}


// pub fn generate_graph_example() -> String {
//     let db: Database<MemoryDatastore> = indradb::MemoryDatastore::new_db();

//     let rome: Vertex = add_vertex(&db, "Rome".to_string());
//     let london: Vertex = add_vertex(&db, "London".to_string());
//     let paris: Vertex = add_vertex(&db, "Paris".to_string());
//     let detroit: Vertex = add_vertex(&db,"Detroit".to_string());


//     //all roads lead to Rome
//     add_edge(&db, &london, &rome, "7".to_string());

//     add_edge(&db, &paris, &rome, "4".to_string());
//     add_edge(&db, &detroit, &paris, "12".to_string());


//     let mut adj_mat : Vec<Vec<usize>> = vec![vec![0; 4]; 4];
    
//     return generate_graph(&db, adj_mat);
// }

pub fn generate_graph_example() -> String {
    let db: Database<MemoryDatastore> = indradb::MemoryDatastore::new_db();

    let v0: Vertex = add_vertex(&db, "0".to_string());
    let v1: Vertex = add_vertex(&db, "1".to_string());
    let v2: Vertex = add_vertex(&db, "2".to_string());
    let v3: Vertex = add_vertex(&db, "3".to_string());

    let mut adj_mat : Vec<Vec<usize>> = vec![vec![0; 4]; 4];

    add_edge(&db, &v0, &v1, "7".to_string());
    adj_mat[0][1] = 1;
    adj_mat[1][0] = 1;
    add_edge(&db, &v1, &v2, "7".to_string());
    adj_mat[1][2] = 1;
    adj_mat[2][1] = 1;
    add_edge(&db, &v1, &v3, "7".to_string());
    adj_mat[1][3] = 1;
    adj_mat[3][1] = 1;
    add_edge(&db, &v2, &v0, "7".to_string());
    adj_mat[2][0] = 1;
    adj_mat[0][2] = 1;
    
    return generate_graph(&db, adj_mat);
}



// pub fn generate_graph_random(number_of_vertices: usize, number_of_edges: usize) -> String {
pub fn generate_graph_random(number_of_vertices: usize, number_of_edges: usize) -> String {

    let db: Database<MemoryDatastore> = indradb::MemoryDatastore::new_db();

    let mut vertex_ids = Vec::new();

    let mut adj_mat : Vec<Vec<usize>> = vec![vec![0; number_of_vertices]; number_of_vertices];

    for i in 0..number_of_vertices {
        let id = add_vertex(&db, i.to_string());
        vertex_ids.push(id);
    }

    for i in 0..number_of_edges {
        let mut out_id: usize;
        let mut in_id: usize;

        loop {
            out_id = rand::thread_rng().gen_range(0..number_of_vertices);
            in_id = rand::thread_rng().gen_range(0..number_of_vertices);
  

            let loop_edge = out_id == in_id;

            let existing_edge = adj_mat[out_id][in_id] == 1; //uni-directional

            //TODO: check condition for max number of edges vs number of vertices
            if loop_edge || existing_edge {
                continue;
            }

            add_edge(&db, &vertex_ids[out_id], &vertex_ids[in_id], "q".to_string());
            adj_mat[out_id][in_id] = 1;
            adj_mat[in_id][out_id] = 1;            

            break
        }
    }

    return generate_graph(&db, adj_mat);
}