/// handles command line arguments, initializes main components, and ties together modules
mod tokenizer;

fn main() {
    let file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/sample.txt";
    let tokens = tokenizer::tokenize(file_path);
    println!("{:?}", tokens);
}
