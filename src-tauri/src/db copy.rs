
use indradb::{self, Vertex, Edge, Identifier };

// Create an in-memory datastore

pub fn test_db() {
    println!("inside");

// Create an in-memory datastore
let db: indradb::Database<indradb::MemoryDatastore> = indradb::MemoryDatastore::new_db();

    // Create a couple of vertices
     let id: Identifier = indradb::Identifier::new(name).unwrap();
     let out: Vertex = Vertex::new(id);
     db.create_vertex(&out).unwrap();
}

// pub fn create_db() -> MemoryDb {
//     // Create an in-memory datastore
//     let db = indradb::MemoryDatastore::new_db();
//     db
// }



// pub fn add_vertex(db: &MemoryDb, name: String)-> Vertex {
//     // Create a couple of vertices
//      let id: Identifier = indradb::Identifier::new(name).unwrap();
//      let out: Vertex = Vertex::new(id);
//      db.create_vertex(&out).unwrap();

//      out
// }

// pub fn add_edge(db: &MemoryDb, out_v: Vertex, in_v: Vertex, label:Identifier) -> Edge {
//     // Add an edge between the vertices
//     let edge = indradb::Edge::new( out_v.id, label, in_v.id);
//     db.create_edge(&edge).unwrap();

//     edge
// }

// pub fn get_edge(db: &MemoryDb, edge: Edge) -> Vec<indradb::QueryOutputValue> {
//     // Query for the edge
//     let id_edge = indradb::SpecificEdgeQuery::single(edge);
//     let output: Vec<indradb::QueryOutputValue> = db.get(id_edge.clone()).unwrap();

//     print!("{:?}", output);

//     output

// }

// pub fn get_vertex(db: &MemoryDb, vertex: Vertex) -> Vec<indradb::QueryOutputValue> {
//     // Query for the edge
//     let id_vertex = indradb::SpecificVertexQuery::single(vertex.id);
//     let output: Vec<indradb::QueryOutputValue> = db.get(id_vertex.clone()).unwrap();

//     print!("{:?}", output);

//     output

// }

// pub fn greet() {

//      println!("hello");
//     // let db = create_db();

//     // let out_v = add_vertex(&db, "Luke Skywalker".to_string());
//     // let in_v = add_vertex(&db, "Leia Organa".to_string());

//     // let edge = add_edge(&db, out_v, in_v, Identifier::new("likes").unwrap());



//     // // Query for the edge
//     // let id_edge = indradb::SpecificEdgeQuery::single(edge);
//     // let output: Vec<indradb::QueryOutputValue> = db.get(id_edge.clone()).unwrap();


//     // // Convenience function to extract out the edges from the query results
//     // let e = indradb::util::extract_edges(output).unwrap();
//     // assert_eq!(e.len(), 1);
//     // // print!("{:?}", e[0]);
//     // println!("{:?}", e);

//     // let origin = Point { x: 0.0, y: 0.0 };
//     // println!("Origin: ({}, {})", origin.x, origin.y);

//     // let json_data = serde_json::to_string(&edge).expect("Failed to serialize to JSON");
//     // println!("{}", json_data);

    
//     // println!("{:?}", edges);   
//     // println!("{:?}", store.datastore);   


//     // // Create an instance of the Json struct
//     // let json_struct = Json(Arc::new(json_value));

//     // // Now you can access the Json struct and its field
//     // println!("{:?}", json_struct.0); // Access the Arc<Value> field
// }




// // pub fn test_connect() {
// //     println!("creating database");

// //     let db = indradb::MemoryDatastore::new_db();

// //     let out_v = add_vertex(&db, "Luke Skywalker".to_string());
// //     let in_v = add_vertex(&db, "Leia Organa".to_string());

// //     // let edge = add_edge(&db, out_v, in_v, Identifier::new("likes").unwrap());
// // }


// //