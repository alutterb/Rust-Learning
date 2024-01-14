/*
Input: Vector of tokens
Output: Dense vector representation of the text

We can either use a pretrained embedding representation or create our own
*/

use ndarray::Array2; // 2D arrays


pub fn load_pretrained_embeddings(file_path: &str) -> Array2<f64> {
    // TODO: Implement
}

pub fn create_embeddings(tokenized_text: &[Vec<String>]) -> Array2<f64> {
    // TODO: Implement
}
