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
//!     "identifier": "dataset1",
//!     "domain": "user-generated"
//!   },
//!   "dataset2": {
//!     "description": {
//!       "en": "Another great dataset"
//!     },
//!     "title": "Dataset Uimhir a Dó",
//!     "links": [],
//!     "identifier": "dataset2",
//!     "domain": "linguistics"
//!   },
use std::collections::HashMap;
use serde::de::{Deserialize,Deserializer,Visitor};
use serde;
use std::fmt;

#[derive(Debug,Deserialize)]
/// A dataset
pub struct Dataset {
    pub description : HashMap<String, Option<String>>,
    pub title : Option<String>,
    pub links : Vec<Link>,
    pub identifier : String,
    pub domain : String,
    pub triples : IntLike
}

/// A link from a dataset to a target dataset
#[derive(Debug,Deserialize)]
pub struct Link {
    pub target : String,
    pub value : String
}

#[derive(Debug,Clone)]
pub struct IntLike(Option<i32>);

impl IntLike {
    pub fn get(&self) -> i32 {
        self.0.unwrap_or(0).clone()
    }
}

impl<'de> Deserialize<'de> for IntLike {
    fn deserialize<D>(deserializer: D) -> Result<IntLike, D::Error>
        where D: Deserializer<'de>
        {
            deserializer.deserialize_any(IntLikeVisitor)
        }
}

struct IntLikeVisitor;

impl<'de> Visitor<'de> for IntLikeVisitor {
    type Value = IntLike;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i8<E>(self, value: i8) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        Ok(IntLike(Some(value as i32)))
    }

    fn visit_i32<E>(self, value: i32) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        Ok(IntLike(Some(value)))
    }

    fn visit_i64<E>(self, value: i64) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        use std::i32;
        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            Ok(IntLike(Some(value as i32)))
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }

    fn visit_u8<E>(self, value: u8) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        Ok(IntLike(Some(value as i32)))
    }

    fn visit_u32<E>(self, value: u32) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        Ok(IntLike(Some(value as i32)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        use std::i32;
        if value <= i32::MAX as u64 {
            Ok(IntLike(Some(value as i32)))
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }


    fn visit_str<E>(self, value :&str) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        Ok(IntLike(value.parse::<i32>().ok()))
    }

    fn visit_string<E>(self, value : String) -> Result<IntLike, E>
        where E: serde::de::Error
    {
        Ok(IntLike(value.parse::<i32>().ok()))
    }


    // Similar for other methods:
    //   - visit_i16
    //   - visit_u8
    //   - visit_u16
    //   - visit_u32
    //   - visit_u64
}
