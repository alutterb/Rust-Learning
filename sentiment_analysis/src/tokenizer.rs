/*
Input - .txt file
Output - Vector of tokens
*/
use std::fs;

fn read_txt_file(file_path: &str) -> String {
    let file_contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        // note - panic! doesn't return a value and is therefore not relevant to the fcns return type
        Err(e) => panic!("Error reading file {}: {}", file_path, e)
    };

    return file_contents
}

pub fn tokenize(file_path: &str) -> Vec<String> {
    let text = read_txt_file(file_path);
    // We will use the simple case of splitting on whitespace
    // Does not take into account punctuation, stop words, etc...
    text.split_whitespace().map(|s| s.to_lowercase()).collect()
    // |s| is a closure, similar to lambda functions in python
    // .collect() will aggregate the iterator into a vector
}