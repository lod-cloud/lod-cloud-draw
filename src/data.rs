//! The readers for the LOD cloud data JSON
//! 
//! # Example
//!
//! json```
//! {
//!   "dataset1": {
//!     "description": {
//!       "en": "A great dataset"
//!     },
//!     "title": "Dataset Numero Uno",
//!     "links": [{
//!        "target": "dataset2",
//!        "value": "456"
//!     }],
//!     "identifier": "dataset1"
//!   },
//!   "dataset2": {
//!     "description": {
//!       "en": "Another great dataset"
//!     },
//!     "title": "Dataset Uimhir a Dó",
//!     "links": [],
//!     "identifier": "dataset2"
//!   },
use std::collections::HashMap;

#[derive(Debug,Serialize,Deserialize)]
/// A dataset
pub struct Dataset {
    pub description : HashMap<String, Option<String>>,
    pub title : Option<String>,
    pub links : Vec<Link>,
    pub identifier : String
}

/// A link from a dataset to a target dataset
#[derive(Debug,Serialize,Deserialize)]
pub struct Link {
    pub target : String,
    pub value : String
}
