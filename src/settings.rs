//! General settings file for creating the cloud diagram
//!
//! # Example
//!
//! json```
//! {
//!    "legend": [
//!        {
//!            "title": "Cross Domain",
//!            "domain": "cross_domain",
//!            "colour": "#c8a788"
//!        },
//!        {
//!            "title": "Geography",
//!            "domain": "geography",
//!            "colour": "#29c9cc"
//!        },
//!        {
//!            "title": "Government",
//!            "domain": "government",
//!            "colour": "#f6b33c"
//!        },
//!        {
//!            "title": "Life Sciences",
//!            "domain": "life_sciences",
//!            "colour": "#db777f"
//!        },
//!        {
//!            "title": "Linguistics",
//!            "domain": "linguistics",
//!            "colour": "#36bc8d"
//!        },
//!        {
//!            "title": "Media",
//!            "domain": "media",
//!            "colour": "#008080"
//!        },
//!        {
//!            "title": "Publications",
//!            "domain": "publications",
//!            "colour": "#f6f3ce"
//!        },
//!        {
//!            "title": "Social Networking",
//!            "domain": "social_networking",
//!            "colour": "#b5b5b5"
//!        },
//!        {
//!            "title": "User Generated",
//!            "domain": "user_generated",
//!            "colour": "#d84d8c"
//!        }
//!    ],
//!    "fixed_points": {},
//!    "selection": "all"
//! }```
use std::collections::HashMap;


/// The Settings object
#[derive(Clone,Debug,PartialEq,Deserialize)]
pub struct Settings {
    pub legend : Vec<LegendEntry>,
    pub fixed_points : HashMap<String, (f64, f64)>,
    pub selection : Option<String>
}

/// A single type of data to be included in the Legend
#[derive(Clone,Debug,PartialEq,Deserialize)]
pub struct LegendEntry {
    /// The display title to be shown to the user
    pub title : String,
    /// The value of the `domain` property in data that corresponds to
    pub domain : Option<String>,
    /// The (HTML) colour of this legend
    pub colour : String
}
