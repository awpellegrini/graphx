
use std::path::PathBuf;
use uuid;
use indradb::{Vertex, Edge, BulkInsertItem, AllVertexQuery, AllEdgeQuery, Identifier};
use tauri::api::path::data_dir;

//db
pub fn get_db_path() -> PathBuf {
    let app_data_path = data_dir().unwrap();
    let db_path = app_data_path.join("graphx.db");

    return db_path;
}

pub fn get_db() -> indradb::Database<indradb::RocksdbDatastore> {
    let db = indradb::RocksdbDatastore::new_db(get_db_path()).unwrap();
    return db
}

pub fn clear_db() {
    let db = get_db();
    db.delete(AllEdgeQuery).unwrap();
    db.delete(AllVertexQuery).unwrap();
}

// //insertions
pub fn create_identifier(label: &str) -> Identifier {
    return Identifier::new(label).unwrap();
}

pub fn create_vertex(label: &str) -> Vertex {
    let db = get_db();

    let db_vertex: Vertex = Vertex::new(create_identifier(label));
    db.create_vertex(&db_vertex).unwrap();
    
    return db_vertex
}

pub fn create_edge(out_v: &str , in_v: &str, label: &str) -> Edge {
    let db = get_db();

    let id: Identifier = create_identifier(label);
    let out_id = uuid::Uuid::parse_str(out_v).unwrap();
    let in_id = uuid::Uuid::parse_str(in_v).unwrap();

    let edge = indradb::Edge::new( out_id, id, in_id);
  
    db.create_edge(&edge).unwrap();

    return edge
}

pub fn delete_vertex(id: &str) {
    let db = get_db();
    let uid = uuid::Uuid::parse_str(id).unwrap();
    let q = indradb::SpecificVertexQuery::new(vec![uid]);
  
    db.delete(q).unwrap();
}

pub fn delete_edge(id: &str) {
    let edges = get_edges();

    let edge = edges.into_iter().find(|x| format!("{} {}", x.outbound_id, x.inbound_id) == id).unwrap();
    let q = indradb::SpecificEdgeQuery::single(edge);

    let db = get_db();
    db.delete(q).unwrap();
}

pub fn create_graph(graph: &(Vec<Vertex>, Vec<Edge>) ) {
    let db = get_db();

    let bulk_vertices = graph.0.iter().map(|x| BulkInsertItem::Vertex(x.clone())).collect::<Vec<BulkInsertItem>>();
    let bulk_edges = graph.1.iter().map(|x| BulkInsertItem::Edge(x.clone())).collect::<Vec<BulkInsertItem>>();

    db.bulk_insert(bulk_vertices).unwrap();
    //probably can do these together
    db.bulk_insert(bulk_edges).unwrap();
}

//queries
pub fn get_vertices() -> Vec<Vertex>{
    let db = get_db();

    let vertices: Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery).unwrap();
    let v = indradb::util::extract_vertices(vertices.to_vec()).unwrap();

    return v;
}

pub fn get_edges() -> Vec<Edge> {
    let db = get_db();

    let edges: Vec<indradb::QueryOutputValue> = db.get(indradb::AllEdgeQuery).unwrap();
    let e = indradb::util::extract_edges(edges).unwrap();

    return e;
}
