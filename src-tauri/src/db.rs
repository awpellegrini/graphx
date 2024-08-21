
use indradb::{self, Vertex, Edge, Identifier };

// Create an in-memory datastore

pub fn test_db() {
    println!("inside");

    // Create an in-memory datastore
    let db: indradb::Database<indradb::MemoryDatastore> = indradb::MemoryDatastore::new_db();


    let id1: Identifier = indradb::Identifier::new("a").unwrap();
    let out_v: Vertex = Vertex::new(id1);
    db.create_vertex(&out_v).unwrap();


    let id2: Identifier = indradb::Identifier::new("b").unwrap();
    let in_v: Vertex = Vertex::new(id2);
    db.create_vertex(&in_v).unwrap();

    let edge = indradb::Edge::new( out_v.id,
        indradb::Identifier::new("likes").unwrap(),
         in_v.id);
    db.create_edge(&edge);


    // let id_edge = indradb::SpecificEdgeQuery::single(edge);
    // let output: Vec<indradb::QueryOutputValue> = db.get(id_edge.clone()).unwrap();
    let output: Vec<indradb::QueryOutputValue> = db.get(indradb::SpecificEdgeQuery::single(edge.clone())).unwrap();
    // Convenience function to extract out the edges from the query results

    let e = indradb::util::extract_edges(output).unwrap();
    println!("{:?}", e[0]);

    println!("come over");
}
