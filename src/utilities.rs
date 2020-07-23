use std::env::current_dir;
use std::collections::HashMap;
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs::File;
use std::io;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub fn get_user_selection(options: &HashMap<&str, &str>) -> String {
    let mut prompt = "Please select from the following options \n\n".to_string();

    for (key, description) in options.iter() {
        prompt.push_str(&format!("{}: {} \n", &key, &description));
    }

    let mut input = String::new();

    while !options.contains_key(input.as_str()) {
        input = get_user_input(&prompt);
    }

    input
}


pub fn get_user_input(prompt: &str) -> String {
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

// DeserizeOwned is different DeserizeOwned
pub fn open_with_relative_path<T>(file: PathBuf) -> T 
    where 
        T: DeserializeOwned
{
    let mut path = current_dir().unwrap();
    path.push(file);
    let mut file = File::open(path).unwrap();
    let mut serialized_string = String::new();
    file.read_to_string(&mut serialized_string).unwrap();
    let a: T = serde_json::from_str(&serialized_string).unwrap();
    a
}