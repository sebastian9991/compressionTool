mod compression;
use std::collections::HashMap;
use std::fs;

fn main() {
    let mut character_counts: HashMap<char, i32> = HashMap::new();
    let binding = fs::read_to_string("data/data.txt")
        .expect("Error: The file should have been able to be read");

    let mut data_string = binding;
    data_string.retain(|c| !c.is_whitespace());
    let vector_characters: Vec<char> = data_string.chars().collect();

    //Map this vector into the hashmap counting the number of occurences of each character
    for c in vector_characters.iter() {
        *character_counts.entry(*c).or_insert(0) += 1;
    }

    println!("Compressing...");
    println!("{:?}", character_counts);
}
