/*
Input - .txt file
Output - Vector of tokens
*/

pub fn tokenize(text: &str) -> Vec<String> {
    // We will use the simple case of splitting on whitespace
    // Does not take into account punctuation, stop words, etc...
    text.split_whitespace().map(|s| s.to_lowercase()).collect()
    // |s| is a closure, similar to lambda functions in python
    // .collect() will aggregate the iterator into a vector
}