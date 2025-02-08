#![warn(missing_docs)]
//! Database stuff for containing recipes and their info.
pub mod error;

use error::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{collections::HashMap, path::Path};

/// Recipe for a ressource.
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    name: String,
    ingredients: Vec<HashMap<String, u16>>,
    cost: f32,
    building: String,
}

/// A building and its usage.
#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    name: String,
    consumption: f32,
}

/// Save building to a file in JSON format.
pub fn save_buildings<P>(path: P, buildings: HashMap<String, Building>) -> Result<(), DatabaseError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if path.is_dir() {
        return Err(DatabaseError::IsDirectory);
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &buildings)?;
    writer.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_test() {
        let mut ingredients_0 = HashMap::<String, u16>::new();
        ingredients_0.insert(String::from("other coal"), 3u16);
        ingredients_0.insert(String::from("coal"), 5u16);

        let mut ingredients_1 = HashMap::<String, u16>::new();
        ingredients_1.insert(String::from("coal"), 8u16);

        let recipe = Recipe {
            name: String::from("compressed coal"),
            ingredients: vec![ingredients_0, ingredients_1],
            cost: 0.0,
            building: String::from("constructor"),
        };

        let j = serde_json::to_string(&recipe).unwrap();

        println!("{}", j);
    }
}
