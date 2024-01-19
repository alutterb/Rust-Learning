/// handles command line arguments, initializes main components, and ties together modules
mod tokenizer;
mod embeddings;
mod sentiment_data_loader;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_text_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/sample.txt";
    let embedding_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/glove.6B/glove.6B.50d.txt";
    let sentiment_data_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/training/training.1600000.processed.noemoticon.csv";

    let tokens = tokenizer::tokenize(raw_text_file_path);
    let glove_embeddings = embeddings::load_glove_embeddings(embedding_file_path)?;
    let sentiment_data = sentiment_data_loader::read_csv(sentiment_data_file_path)?;

    // prints alot... need to slice it
    println!("Sentiment Data: {:?}", sentiment_data);


 Ok(())   
}
