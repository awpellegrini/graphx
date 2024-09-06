use indradb::{Vertex, Edge};

// depth first search
fn dfs(vertices: &Vec<String>, edges: &Vec<(String, String)>, initial_vertex: String) -> Vec<String> {
    let mut index_stack = vec!();
    index_stack.push(initial_vertex);
    
    let mut discovered = vertices.into_iter().map(|x| (x.to_string(), false)).collect::<Vec<(String, bool)>>();
    let mut i = 0;

    while index_stack.len() > 0 && i<1000 {
        let v = index_stack.pop().unwrap();
        let index_of_v = discovered.iter_mut().position(|r|  r.0.to_string() == v).unwrap();
        
        if !discovered[index_of_v].1 {
            discovered[index_of_v].1 = true;

            for edge in edges.iter() {
                if edge.0 == v {
                    index_stack.push(edge.1.clone());
                }
            }
        }

        i+=1;
    }
    //return just the id's
    return discovered.into_iter().filter(|x| x.1 == true).map(|x| x.0.to_string()).collect::<Vec<String>>();
}

pub fn get_connected_subgraph(vertices: Vec<Vertex>, edges: Vec<Edge>, vertex_id: String) -> (Vec<String>, Vec<(String, String)>) {
    let vertex_ids = get_vertex_ids(vertices);
    let edge_ids = get_edge_ids(edges);

    let result = dfs(&vertex_ids, &edge_ids, vertex_id);
    let filtered_edges = edge_ids.into_iter().filter(|x| result.contains(&x.0)).collect::<Vec<(String, String)>>();

    return (result, filtered_edges);
}

fn get_vertex_ids(vertices: Vec<Vertex>) -> Vec<String>{
    let ids: Vec<String>= vertices.into_iter().map(|x| x.id.to_string()).collect::<Vec<String>>();
    return ids;
}

fn get_edge_ids(edges: Vec<Edge>) -> Vec<(String, String)>{
    let edge_ids = edges.into_iter()
        .map(|x| (x.outbound_id.to_string(), x.inbound_id.to_string()))
        .collect::<Vec<(String, String)>>();
    
    return edge_ids;
}