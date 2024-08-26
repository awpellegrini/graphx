use indradb::{Vertex, Edge};
#[path = "../db.rs"] mod db;

//RANDOM
use rand::Rng; // 0.8
pub fn create_graph_random(number_of_vertices: usize, number_of_edges: usize) -> ((Vec<Vertex>, Vec<Edge>), Vec<Vec<usize>>) {
    let mut vertices =  Vec::new();
    let mut edges =  Vec::new();
    let mut adj_mat : Vec<Vec<usize>> = vec![vec![0; number_of_vertices]; number_of_vertices];

    for i in 0..number_of_vertices {
        let vertex = Vertex::new(indradb::Identifier::new(i.to_string()).unwrap());
        vertices.push(vertex);
    }

    for _j in 0..number_of_edges {
        let mut out_id: usize;
        let mut in_id: usize;

        loop {
            out_id = rand::thread_rng().gen_range(0..number_of_vertices);
            in_id = rand::thread_rng().gen_range(0..number_of_edges);
  
            let loop_edge = out_id == in_id;

            let existing_edge = adj_mat[out_id][in_id] == 1; //uni-directional


        // n(n-1)/2 and obviously in a directed graph there are twice as many.
        // if i >= 1 * (number_of_vertices * (number_of_vertices - 1) / 2) {
        //     break;
        // }
            //TODO: check condition for max number of edges vs number of vertices
            if loop_edge || existing_edge {
                continue;
            }
 

            let edge = Edge::new(
                vertices[out_id].id, 
                indradb::Identifier::new("edge").unwrap(), 
                vertices[in_id].id);
            edges.push(edge);

            adj_mat[out_id][in_id] = 1;
            // adj_mat[in_id][out_id] = 1;            

            break
        }
    }

    println!("{:?}", edges);

    return ((vertices, edges), adj_mat);
}

pub fn generate_graph_random(vertices: usize, edges: usize) -> ((Vec<Vertex>, Vec<Edge>), Vec<Vec<usize>>) {
    let (random_graph, adj_mat) = create_graph_random(vertices, edges);
    db::create_graph(&random_graph);

    return (random_graph, adj_mat);
}

//STATIC EXAMPLE
fn create_graph_example() -> (Vec<Vertex>, Vec<Edge>) {
    let rome = indradb::Vertex::new(indradb::Identifier::new("rome").unwrap());
    let paris = indradb::Vertex::new(indradb::Identifier::new("paris").unwrap());

    let paris_to_rome = indradb::Edge::new(paris.id, indradb::Identifier::new("paris_to_rome").unwrap(), rome.id);

    let vertices = vec![rome, paris];
    let edges = vec![paris_to_rome];

    return (vertices, edges);

}
pub fn generate_graph_example() -> ((Vec<Vertex>, Vec<Edge>), Vec<Vec<usize>>) {
    let example = create_graph_example();
    db::create_graph(&example);

    return (example, [].to_vec());
}
