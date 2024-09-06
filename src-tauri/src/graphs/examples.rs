use std::cmp::min;
use indradb::{Vertex, Edge};
use rand::Rng; // 0.8

pub fn create_graph_random(number_of_vertices: usize, number_of_edges: usize, directed: bool) -> ((Vec<Vertex>, Vec<Edge>), Vec<Vec<usize>>) {
    let mut vertices =  Vec::new();
    let mut edges =  Vec::new();
    let mut adj_mat : Vec<Vec<usize>> = vec![vec![0; number_of_vertices]; number_of_vertices];
    
    //create vertices
    for i in 0..number_of_vertices {
        //TODO: use db util for identifier
        let vertex = Vertex::new(indradb::Identifier::new(i.to_string()).unwrap());
        vertices.push(vertex);
    }

    let max_edges = if directed {
            number_of_vertices * (number_of_vertices - 1)
        } else {
            number_of_vertices * (number_of_vertices - 1) / 2
        };

    let max_loops = min(number_of_edges, max_edges);

    for _j in 0..max_loops  {
        let mut out_id: usize;
        let mut in_id: usize;

        loop {
            out_id = rand::thread_rng().gen_range(0..number_of_vertices);
            in_id = rand::thread_rng().gen_range(0..number_of_vertices);
  
            let loop_edge = out_id == in_id;
            let existing_edge ;

            if directed {
                existing_edge = adj_mat[out_id][in_id] == 1;
            } else {
                existing_edge = adj_mat[out_id][in_id] == 1 || adj_mat[in_id][out_id] == 1;
            }

            if loop_edge || existing_edge {
                continue;
            }

            //create vertices
            if directed {
                let edge = Edge::new(
                    vertices[out_id].id, 
                    indradb::Identifier::new("edge").unwrap(), 
                    vertices[in_id].id);

                adj_mat[out_id][in_id] = 1;
                edges.push(edge);
            } else {
                let out_edge = Edge::new(
                    vertices[out_id].id, 
                    indradb::Identifier::new("edge").unwrap(), 
                    vertices[in_id].id);

                
                let in_edge = Edge::new(
                    vertices[in_id].id, 
                    indradb::Identifier::new("edge").unwrap(), 
                    vertices[out_id].id);
          
                edges.push(out_edge);
                edges.push(in_edge);

                adj_mat[out_id][in_id] = 1;
                adj_mat[in_id][out_id] = 1;
            }

            break
        }
    }

    return ((vertices, edges), adj_mat);
}

//STATIC EXAMPLE
pub fn create_graph_example() -> ((Vec<Vertex>, Vec<Edge>), Vec<Vec<usize>>) {
    let rome = indradb::Vertex::new(indradb::Identifier::new("rome").unwrap());
    let paris = indradb::Vertex::new(indradb::Identifier::new("paris").unwrap());

    let paris_to_rome = indradb::Edge::new(paris.id, indradb::Identifier::new("paris_to_rome").unwrap(), rome.id);

    let vertices = vec![rome, paris];
    let edges = vec![paris_to_rome];

    //no adj mat in this case
    return ((vertices, edges), [].to_vec())

}