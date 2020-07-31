use std::env::current_dir;
use std::collections::HashMap;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::File;
use std::hash::Hash;
use std::io;

use serde::Serialize;
use serde::de::DeserializeOwned;

// pub fn get

pub fn get_user_input_line(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    input = input.trim().to_string();
    input
}

pub fn save_with_relative_path(object: &impl Serialize, file: PathBuf) {
    let string = serde_json::to_string(object).unwrap();
    let mut path = current_dir().unwrap();
    path.push(file);
    let mut file = File::create(path).unwrap();
    file.write_all(string.as_bytes()); // as bytes converts it to byte array
}

// DeserizeOwned is different Deserize
pub fn deserialize_from_relative_path<T>(file: PathBuf) -> T
    where 
        T: DeserializeOwned
{
    let serialized_string = load_with_relative_path(file);
    let a: T = serde_json::from_str(&serialized_string).unwrap();
    a
}

pub fn load_with_relative_path(file: PathBuf) -> String {
    let mut path = current_dir().unwrap();
    path.push(file);
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    string
}

pub fn map_a_hash_map<T,U, V, W>(original: &HashMap<T,U>, key_value_mapper: fn((&T,&U)) -> (V,W) ) -> HashMap<V,W> 
    where 
        V: Hash + Eq + PartialEq,
{
    let mut new_hash_map: HashMap<V,W> = HashMap::new();

    for original_key_vlue_pair in original {
        let new_key_vlue_pair = key_value_mapper(original_key_vlue_pair);
        new_hash_map.insert(new_key_vlue_pair.0, new_key_vlue_pair.1);
    };

    new_hash_map
}