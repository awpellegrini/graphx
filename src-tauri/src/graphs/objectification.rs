
use indradb::{Vertex, Edge};
use serde_json::{Value, json};

pub fn objectify(graph: &(Vec<Vertex>, Vec<Edge>)) -> Value {
    let vertices = graph.0.iter().map(|v| {
        json!({
            "id": v.id,
            "label": v.t
        })
    }).collect::<Vec<Value>>();

    let edges = graph.1.iter().map(|e| {
        json!({
            "label": e.t,
            "outbound_id": e.outbound_id,
            "inbound_id": e.inbound_id,
        })
    }).collect::<Vec<Value>>();

    return json!({
            "vertices": vertices,
            "edges": edges
    });
}

