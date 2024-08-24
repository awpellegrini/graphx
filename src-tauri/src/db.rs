
use indradb::{AllVertexQuery, Database, Edge, Identifier, MemoryDatastore, Vertex};


use once_cell::sync::Lazy;

// static db: Lazy<Database<MemoryDatastore>> = Lazy::new(indradb::MemoryDatastore::new_db);

pub fn tester() {
    println!("test");
}

pub fn add_vertex(db: &indradb::Database<MemoryDatastore>, label: String)-> Vertex {
     let id: Identifier = indradb::Identifier::new(label).unwrap();
     let vertex: Vertex = Vertex::new(id);
     db.create_vertex(&vertex).unwrap();
     
     vertex
}

pub fn add_edge(db: &indradb::Database<MemoryDatastore>, out_v: &Vertex, in_v: &Vertex, label: String) -> Edge {
    let id: Identifier = indradb::Identifier::new(label).unwrap();

    // Add an edge between the vertices
    let edge = indradb::Edge::new( out_v.id, id, in_v.id);
    db.create_edge(&edge).unwrap();

    edge
}


pub fn get_vertices(db: &indradb::Database<MemoryDatastore>) -> Vec<String>{
    let vertices: Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery).unwrap();
    let v = indradb::util::extract_vertices(vertices.to_vec()).unwrap();
    // println!("{:?}", vertices);
    
    let ids: Vec<String>= v.into_iter().map(|x| x.id.to_string()).collect::<Vec<String>>();
    
    return ids; 
}

pub fn get_edges(db: &indradb::Database<MemoryDatastore>) -> Vec<String> {
    let edges: Vec<indradb::QueryOutputValue> = db.get(indradb::AllEdgeQuery).unwrap();
    // println!("{:?}", edges);
    
    let e = indradb::util::extract_edges(edges).unwrap();
    // println!("{:?}", e);
    
    // let sss= indradb::Identifier::new("s").unwrap();
    // println!("{:?}", sss);

    fn createEdgeId(outbound_id: String, inbound_id: String) -> String {
        format!("{}_{}", outbound_id, inbound_id)
    }

    let ids: Vec<String>= e
        .into_iter()
        .map(|x| createEdgeId(x.outbound_id.to_string(), x.inbound_id.to_string()))
        .collect::<Vec<String>>();

    return ids;
}
// pub fn get_vertices(db: &indradb::Database<MemoryDatastore>){
//     let vertices: Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery).unwrap();
//     let v = indradb::util::extract_vertices(vertices.to_vec()).unwrap();

//     println!("{:?}", v);


//     let vId = v[0].id;
//     println!("{:?}", vId);

//     let ids = vec![vId];


//     // let vertices2: Vec<indradb::QueryOutputValue> = db.get(indradb::CountQuery::new(inner)).unwrap();


//     // let q : Vec<indradb::QueryOutputValue> = db.get(indradb::AllVertexQuery).unwrap();

//     let q = db.get(indradb::SpecificVertexQuery{ids:ids}).unwrap();
//     // let vert: Vec<indradb::QueryOutputValue> = db.get(q).unwrap();
//     println!("{:?}",q);

    
//     let idzz: Identifier = indradb::Identifier::new("-1").unwrap(); 
//     let verttry: Vertex = Vertex::new(idzz);
//     db.create_vertex(&verttry).unwrap();
//     let idomg = verttry.id.clone();

//     let visited = false;

//     struct Whatever {
//         visited: bool
//     }


//     let identi = indradb::Identifier::new("visited").unwrap();

//     let jsonv = serde_json::json!({
//         "visited": visited
//     });

//     let json = indradb::Json::new(jsonv);

//     let property = indradb::NamedProperty::new(identi, json);


//     let vertwithprops = indradb::VertexProperties::new(verttry,[property].to_vec()) ;
//     // let id: Identifier = indradb::Identifier::new(label).unwrap();
//     // let vertex: Vertex = Vertex::new(id);
//     // db.create_vertex(&vertex).unwrap();


//     let ids2 = vec![idomg];

//     let q = db.get(indradb::SpecificVertexQuery{ids:ids2}).unwrap();
//     // let vert: Vec<indradb::QueryOutputValue> = db.get(q).unwrap();

//     println!("!{:?}",q);


//     let ssd = db.get(indradb::VertexWithPropertyValueQuery{name: identi, value: mmm}).unwrap();
//     println!("!{:?}",ssd);



// }