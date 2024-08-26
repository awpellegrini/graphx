
use indradb::{Vertex, Edge, BulkInsertItem};


pub fn create_graph(graph: &(Vec<Vertex>, Vec<Edge>) ) {
    let bulk_vertices = graph.0.iter().map(|x| BulkInsertItem::Vertex(x.clone())).collect::<Vec<BulkInsertItem>>();
    let bulk_edges = graph.1.iter().map(|x| BulkInsertItem::Edge(x.clone())).collect::<Vec<BulkInsertItem>>();

    let database = indradb::MemoryDatastore::new_db();
    database.bulk_insert(bulk_vertices).unwrap();
    //probably can do these together
    database.bulk_insert(bulk_edges).unwrap();
}