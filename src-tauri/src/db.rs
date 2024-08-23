
use indradb::{MemoryDatastore, Vertex, Edge, Identifier };

// pub fn create_db() -> indradb::Database<MemoryDatastore> {
//     // Create an in-memory datastore
//     let db = indradb::MemoryDatastore::new_db();
//     db
// }

pub fn add_vertex(db: &indradb::Database<MemoryDatastore>, label: String)-> Vertex {
     let id: Identifier = indradb::Identifier::new(label).unwrap();
     let vertex: Vertex = Vertex::new(id);
     db.create_vertex(&vertex).unwrap();
     
     vertex
}

pub fn add_edge(db: &indradb::Database<MemoryDatastore>, out_v: &Vertex, in_v: &Vertex, label: String) -> Edge {
    let id: Identifier = indradb::Identifier::new(label).unwrap();

    // Add an edge between the vertices
    let edge = indradb::Edge::new( out_v.id, id, in_v.id);
    db.create_edge(&edge).unwrap();

    edge
}