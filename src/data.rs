use std::collections::HashMap;

#[derive(Debug,Serialize,Deserialize)]
pub struct Dataset {
    pub description : HashMap<String, Option<String>>,
    pub title : Option<String>,
    pub links : Vec<Link>,
    pub identifier : String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Link {
    pub target : String,
    pub value : String
}
