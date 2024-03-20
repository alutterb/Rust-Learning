/// handles command line arguments, initializes main components, and ties together modules
mod tokenizer;
mod embeddings;
mod sentiment_data_loader;
mod utils;
mod model;

use std::error::Error;
use ndarray::Array1;
use tch::{nn,Device};

fn main() -> Result<(), Box<dyn Error>> {
    // We will just have one document - an excerpt from a book
    let raw_text_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/sample.txt";
    let embedding_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/glove.6B/glove.6B.50d.txt";
    let sentiment_data_file_path = "/home/akagi/home/Rust-Learning/Rust-Learning/sentiment_analysis/data/training/training.1600000.processed.noemoticon.csv";

    let mut tokens = tokenizer::tokenize(raw_text_file_path);
    let mut glove_embeddings = embeddings::load_glove_embeddings(embedding_file_path)?;
    let mut sentiment_data = sentiment_data_loader::read_csv(sentiment_data_file_path)?;
    
    /* 
    println!("First 100 tokens: {:?}", &tokens[..std::cmp::min(100, tokens.len())]);
    for (i, (key, value)) in glove_embeddings.iter().enumerate().take(2) {
        println!("{:?}: {:?}", key, value);
    }*/

    /*
    Snippet of token and glove embeddings output:
    Tokens - 
    First 100 tokens: ["in", "german", "the", "word", "wild", "denotes", "a", "feral", "animal,", "especially", "one", "hunted", "as", "game,", "and", "sometimes", "it", "specifies", "such", "animals", "as", "deer.", "in", "addition", "it", "connotes", "wildness", "and", "wilderness,", "since", "the", "adjective", "'", "wild'", "exists", "in", "german", "as", "well", "as", "english.", "furthermore,", "it", "is", "probably", "etymologically", "related", "to", "the", "similar", "word", "wald", "(forest)", ".", "this", "network", "of", "associations", "seems", "impossible", "even", "to", "approach", "in", "translation.", "such", "difficulties", "are", "particularly", "frustrating", "inasmuch", "as", "this", "translation", "must", "bear", "almost", "the", "entire", "weight", "of", "trakl's", "exploration", "of", "animality,", "and", "the", "further", "stresses", "of", "heidegger's", "response", "to", "it.", "6", "ibid.,", "48;", "g.", "trakl,", "das"]

    Glove embeddings - 
    "wardensville": [-0.92093, -0.4745, 0.33279, -1.225, -0.30843, -0.55159, 0.37369, 0.70794, 0.44586, 0.073673, -0.23665, -0.95631, 0.55395, -0.32501, -1.679, 0.29799, 0.18537, -0.0089828, 0.080296, 0.72049, 0.10633, 0.020906, -0.17313, 0.64093, 0.069739, 0.38293, 1.1941, 0.2291, 0.52928, -0.27107, -1.413, 0.27994, 0.4881, -0.51271, 0.93743, -0.4658, -0.40783, 0.25143, -0.62992, 0.040518, -0.34164, -0.58023, 0.29205, -0.25903, -0.34812, -0.08503, 1.1639, -0.11736, -0.30876, 0.52057], shape=[50], strides=[1], layout=CFcf (0xf), const ndim=1
    "marketed": [0.56744, -0.95401, -0.13958, -0.36946, -0.65865, 0.77634, -1.0024, -1.5606, 0.36346, 1.0474, 1.0655, 0.57933, 0.10407, -0.53846, 0.070808, 0.62976, -0.57783, 1.0144, 0.086067, -0.44557, 0.64743, -0.49589, 0.54856, 0.68358, -0.90653, -0.58738, -0.47734, 0.096323, 0.062259, -0.32944, 1.3742, -0.22656, 0.32191, -0.27571, 0.56267, -0.26096, 0.020275, 0.84584, -1.0147, -0.52951, 0.54456, -0.50368, 0.32259, 0.24942, 0.46444, 0.34416, -0.6035, -0.18317, -0.24426, 0.32774], shape=[50], strides=[1], layout=CFcf (0xf), const ndim=1
     */

    //println!("Sentiment data snippet: {:?}", &sentiment_data[..std::cmp::min(3, sentiment_data.len())]);

    /*
    Snippet of sentiment data - 
    [SentimentData { target: "0", id: "1467810369", date: "Mon Apr 06 22:19:45 PDT 2009", flag: "NO_QUERY", user: "_TheSpecialOne_", text: "@switchfoot http://twitpic.com/2y1zl - Awww, that's a bummer.  You shoulda got David Carr of Third Day to do it. ;D" }, SentimentData { target: "0", id: "1467810672", date: "Mon Apr 06 22:19:49 PDT 2009", flag: "NO_QUERY", user: "scotthamilton", text: "is upset that he can't update his Facebook by texting it... and might cry as a result  School today also. Blah!" }, SentimentData { target: "0", id: "1467810917", date: "Mon Apr 06 22:19:53 PDT 2009", flag: "NO_QUERY", user: "mattycus", text: "@Kenichan I dived many times for the ball. Managed to save 50%  The rest go out of bounds" }]
    */
    
    // note - here we are using the entire sample.txt as one document
    // TODO - let's create documents on a paragraph level rather than taking all of sample.txt as one document
    let mut model_input: Array1<f64> = utils::generate_model_input(tokens, glove_embeddings);

    let vs = nn::VarStore::new(Device::Cpu);
    let model = model::CNN::new(&vs.root());

    //println!("Input: {:?}", model_input);

 Ok(())   
}
