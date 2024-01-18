/*
Input: Vector of tokens
Output: Dense vector representation of the text

We can either use a pretrained embedding representation or create our own
For simplicity, we will use a pretrained embedding representation
Later on, we will create our own embedding representation in another project
*/

use ndarray::Array1; // 1D arrays
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::collections::HashMap;

pub fn load_glove_embeddings(embedding_path: &str) -> Result<HashMap<String, Array1<f64>>, Box<dyn Error>> {
    // ? before ; is used for error handling
    // File::open returns a Result object, and ? unpacks it to the actual value
    let file = File::open(embedding_path)?;
    let reader = BufReader::new(file);

    let mut embeddings = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        // this line splits the string line into an iterator of substrings with whitespace
        // as the delimiter. Each substring is either the word or the numeric values
        // making up the vector representation of the word
        // we call this variable mutable since we will use the .next() method
        // which will change its internal state by advancing the iterator
        let mut parts = line.split_whitespace();

        // for GLoVE embedding format, the part of iterator will be the word itself
        // ok_or handles the case where .next() returns None
        // and ? will propagate the error or unpack the word value
        // lastly .to_string converts it from the &str slice to an owned String type
        let word = parts.next().ok_or("No word found")?.to_string();

        // Here we parse the embedding representation of the word
        // we use closure to parse the value as a f64
        // and if parsing fails, it is handled by unwrap_or and replaces the value with 0.0
        // and lastly collect() will aggregate the iterator into a vector
        let values: Vec<f64> = parts.map(|p| p.parse::<f64>().unwrap_or(0.0)).collect();

        // now we can insert the word and values into the hashmap
        embeddings.insert(word, Array1::from(values));
    }

    Ok(embeddings)
}