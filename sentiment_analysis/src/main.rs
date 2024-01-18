/// handles command line arguments, initializes main components, and ties together modules
mod tokenizer;
mod embeddings;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_text_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/sample.txt";
    let embedding_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/glove.6B/glove.6B.50d.txt";

    let tokens = tokenizer::tokenize(raw_text_file_path);
    let glove_embeddings = embeddings::load_glove_embeddings(embedding_file_path)?;

    // test on a single word
    let word = "hello";
    if let Some(embedding) = glove_embeddings.get(word) {
        println!("Embedding for {}: {:?}", word, embedding);
    }
    else {
        println!("No embedding found for {}", word);
    }
 Ok(())   
}
