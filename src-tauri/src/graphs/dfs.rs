// #[path = "../db.rs"] mod db;
use indradb::{self, AllVertexQuery};

pub fn get_connected_subgraph(vertex_id: String){
    println!("get_connected_subgraph {:?}", vertex_id);

//    let db = indradb::Me

    // indradb::Database::get(&self, q)
    // let vertices: Vec<indradb::QueryOutputValue> = indradb::MemoryDatastore.get(indradb::AllVertexQuery).unwrap();
    // let v = indradb::util::extract_vertices(vertices.to_vec()).unwrap();
    // println!("{:?}", vertices);
    
    // let ids: Vec<String>= v.into_iter().map(|x| x.id.to_string()).collect::<Vec<String>>();
    // println!("{:?}", ids);
    

    // db::get_vertices();
    // tester();
}



#[derive(Debug)]
pub struct dfs_vertex {
    id: usize,
    visited: bool,
}
#[derive(Debug)]
pub struct dfs_graph {
    vertices: Vec<dfs_vertex>,
    edges: Vec<(usize, usize)>,
}


// pub

pub fn dfs(graph: dfs_graph, initialId: usize){
    let mut vertices = graph.vertices;
    let edges = graph.edges;

    let mut index_stack: Vec<usize> = vec![];

    index_stack.push(initialId);

    let mut i =0;
    // let mut discovered = [false, false, false, false];

    while index_stack.len() > 0 && i<100 {
        println!("*,{}",i);
        let v = index_stack.pop().unwrap();
//        If ( b == value)
//        Return true // we found the value
        println!("popped {:?}",v);


        if !vertices[v].visited {
            vertices[v].visited = true;

            for edge in edges.iter(){
                if edge.0 == vertices[v].id {
                    index_stack.push(edge.1);
                }
            }
        }

        i+=1;
    }
}

// --- //
// #[derive(Debug)]
// pub struct dfs_vertex {
//     id: usize,
//     visited: bool,
// }
// #[derive(Debug)]
// pub struct dfs_graph {
//     vertices: Vec<dfs_vertex>,
//     edges: Vec<(usize, usize)>,
// }


// // pub fn dfs_recursive(G: dfs_graph, v: dfs_vertex){
//     v.visited = true;

//     for edge in G.edges.iter(){
//         if edge.0 == v.id && !G.vertices[edge.1].visited  {
//                 dfs_recursive(G, G.vertices[edge.1]);
//         }
//     }

// }

// pub fn dfs(graph: dfs_graph, initialId: usize){
//     let mut vertices = graph.vertices;
//     let edges = graph.edges;

//     let mut index_stack: Vec<usize> = vec![];

//     index_stack.push(initialId);

//     let mut i =0;
//     // let mut discovered = [false, false, false, false];

//     while index_stack.len() > 0 && i<100 {
//         println!("*,{}",i);
//         let v = index_stack.pop().unwrap();
// //        If ( b == value)
// //        Return true // we found the value
//         println!("popped {:?}",v);


//         if !vertices[v].visited {
//             vertices[v].visited = true;

//             for edge in edges.iter(){
//                 if edge.0 == vertices[v].id {
//                     index_stack.push(edge.1);
//                 }
//             }
//         }

//         i+=1;
//     }

//     println!("xxx{:?}", vertices);
// }

// type VertexType = usize;


// #[derive(Debug)]
// pub struct GraphType {
//     vertices: Vec<VertexType>,
//     edges: Vec<(usize, usize)>,
// }

// pub fn dfs(graph: GraphType, v0: VertexType){
//     let vertices = graph.vertices;
//     let edges = graph.edges;

//     let mut discovered = vec![false; vertices.len()];

//     let mut index_stack: Vec<usize> = vec![];

//     index_stack.push(v0);

//     let mut i =0;

//     while index_stack.len() > 0 && i < 100 {
//         println!("*,{}",i);
//         let v = index_stack.pop().unwrap();
// //        If ( b == value)
// //        Return true // we found the value


//         if !discovered[v] {
//             discovered[v] = true;

//             for edge in edges.iter(){
//                 if edge.0 == vertices[v] {
//                     index_stack.push(edge.1);
//                 }
//             }
//         }

//         i+=1;
//     }

//     println!("xxx{:?}{:?}", vertices, discovered);
// }


