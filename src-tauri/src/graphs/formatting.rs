use indradb;

mod types;

// to avoid naming conflicts
pub type GraphxGraph = types::GraphxGraph;
pub type GraphxVertex = types::GraphxVertex;


pub fn format_vertices_from_db(vertices: &Vec<indradb::QueryOutputValue>) -> Vec<types::GraphxVertex> {
    let v = indradb::util::extract_vertices(vertices.to_vec()).unwrap();
    let vertices_count = v.len();

    //bad
    let mut formatted_vertices = Vec::with_capacity(100);

    for i in 0..vertices_count {
        // maybe borrow?
        let vertex = v[i].clone();

        let formatted_vertex = types::GraphxVertex {
            id: vertex.id.to_string(),
            label:  vertex.t.to_string(),
        };

        formatted_vertices.push(formatted_vertex);
    }

    formatted_vertices
}

pub fn format_edges_from_db(edges: Vec<indradb::QueryOutputValue>) -> Vec<types::GraphxEdge> {
    let e = indradb::util::extract_edges(edges).unwrap();
    let edges_count = e.len();

    //bads
    let mut formatted_edges = Vec::with_capacity(100);

    for i in 0..edges_count {
        //maybe borrow?
        let edge = e[i].clone();

        let formatted_edge = types::GraphxEdge {
            id: format!("{}{}", edge.outbound_id, edge.inbound_id),
            source: edge.outbound_id.to_string(),
            target: edge.inbound_id.to_string(),
            label: edge.t.to_string(),
            size: 3,
        };

        formatted_edges.push(formatted_edge);
    }

    formatted_edges
}
