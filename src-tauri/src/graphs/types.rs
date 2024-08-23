
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GraphxVertex {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct GraphxEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: String,
    pub size: i32,
}


#[derive(Debug, Serialize)]
pub struct GraphxGraph{
    pub vertices: Vec<GraphxVertex>,
    pub edges: Vec<GraphxEdge>
}

