//! General settings file for creating the cloud diagram
//!
//! # Example
//!
//! json```
//! {
//!   "legend": [{
//!     "title": "Cross Domain",
//!     "id": "cross-domain",
//!     "color": "#c8a788"
//!   },{
//!
//!   }]
//! }


/// The Settings object
#[derive(Clone,Debug,PartialEq)]
pub struct Settings {
    pub legend : Vec<LegendEntry>
}

impl Settings {
    /// The LOD cloud settings
    pub fn default() -> Settings {
        Settings {
            legend: [
                LegendEntry::new(
                    "Cross Domain",
                    "cross_domain",
                    "#c8a788",
                ),
                LegendEntry::new(
                    "Geography",
                    "geography",
                    "#29c9cc",
                ),
                LegendEntry::new(
                    "Government",
                    "government",
                    "#f6b33c",
                ),
                LegendEntry::new(
                    "Life Sciences",
                    "life_sciences",
                    "#db777f",
                ),
                LegendEntry::new(
                    "Linguistics",
                    "linguistics",
                    "#36bc8d",
                ),
                LegendEntry::new(
                    "Media",
                    "media",
                    "#008080",
                ),
                LegendEntry::new(
                    "Publications",
                    "publications",
                    "#f6f3ce",
                ),
                LegendEntry::new(
                    "Social Networking",
                    "social_networking",
                    "#b5b5b5",
                ),
                LegendEntry::new(
                    "User Generated",
                    "user_generated",
                    "#d84d8c",
                )
            ].to_vec()
        }
    }
}

/// A single type of data to be included in the Legend
#[derive(Clone,Debug,PartialEq)]
pub struct LegendEntry {
    /// The display title to be shown to the user
    pub title : String,
    /// The value of the `domain` property in data that corresponds to
    pub id : String,
    /// The (HTML) colour of this legend
    pub colour : String
}

impl LegendEntry {
    fn new(title : &str, id : &str, colour : &str) -> LegendEntry {
        LegendEntry {
            title: title.to_string(),
            id: id.to_string(),
            colour: colour.to_string()
        }
    }
}
