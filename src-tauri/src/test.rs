
use indradb::{Edge, Vertex, Database, MemoryDatastore};
#[path = "db.rs"] mod db;
use db::{add_vertex, add_edge, get_vertices, get_edges};

pub fn generate_graph() {
    let db: Database<MemoryDatastore> = indradb::MemoryDatastore::new_db();

    let v0: Vertex = add_vertex(&db, "0".to_string());
    let v1: Vertex = add_vertex(&db, "1".to_string());
    let v2: Vertex = add_vertex(&db, "2".to_string());
    let v3: Vertex = add_vertex(&db, "3".to_string());

    let mut adj_mat : Vec<Vec<usize>> = vec![vec![0; 4]; 4];

    add_edge(&db, &v0, &v1, "7".to_string());
    adj_mat[0][1] = 1;
    adj_mat[1][0] = 1;
    add_edge(&db, &v1, &v2, "7".to_string());
    adj_mat[1][2] = 1;
    adj_mat[2][1] = 1;
    add_edge(&db, &v1, &v3, "7".to_string());
    adj_mat[1][3] = 1;
    adj_mat[3][1] = 1;
    add_edge(&db, &v2, &v0, "7".to_string());
    adj_mat[2][0] = 1;
    adj_mat[0][2] = 1;

}



#[path = "graphs/dfs.rs"] mod dfs;

pub fn testconnect() {
    println!("****START****");
    db::tester()
    // generate_graph();
    // dfs::tester();

    // let db: Database<MemoryDatastore> = indradb::MemoryDatastore::new_db();

    // let v0: Vertex = add_vertex(&db, "0".to_string());
    // let v1: Vertex = add_vertex(&db, "1".to_string());
    // let v2: Vertex = add_vertex(&db, "2".to_string());
    // let v3: Vertex = add_vertex(&db, "3".to_string());

    // let mut adj_mat : Vec<Vec<usize>> = vec![vec![0; 4]; 4];

    // add_edge(&db, &v0, &v1, "a".to_string());
    // adj_mat[0][1] = 1;
    // adj_mat[1][0] = 1;
    // add_edge(&db, &v1, &v2, "b".to_string());
    // adj_mat[1][2] = 1;
    // adj_mat[2][1] = 1;
    // add_edge(&db, &v1, &v3, "c".to_string());
    // adj_mat[1][3] = 1;
    // adj_mat[3][1] = 1;
    // add_edge(&db, &v2, &v0, "d".to_string());
    // adj_mat[2][0] = 1;
    // adj_mat[0][2] = 1;

    // let verts = get_vertices(&db);
    // println!("111{:?}", verts);
    


    // let edges = get_edges(&db);
    // println!("222{:?}", edges);

    // format!("{}", "test")
}


