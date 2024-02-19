// This file will contain helper functions to be used in other files
use std::collections::HashMap;
use ndarray::Array1;

pub fn generate_model_input(tokens: Vec<String>, embeddings: HashMap<String, Array1<f64>>) -> Array1<f64> {
    /*
        Aggregates embeddings for each token in a given document to be used as input
        for the model to generate a prediction

        # Example
        Suppose we have the sentence "this cat is cool". This shall be our only document.
        We tokenize the text, the find its vector embedding representation.
        Let's say this = [0,1,1,0], cat = [0,0,1,1], is = [0,0,0,1], cat = [1,0,0,0]

        The model will only take a single vector embedding, propogate it through the network,
        and then generate a final prediction. Therefore, we must combine these embeddings to
        create only a single embedding. For the sake of this tutorial, we simply add them together
        model_input = [0,1,1,0] + [0,0,1,1] + [0,0,0,1] + [1,0,0,0] = [1,1,2,2]

        # Parameters
        `tokens` - list of tokens in a document
        `embeddings` - pretrained embeddings to grab vector embedding for each token

        # Returns
        The Array1<f64> aggregated embedding representation of the document to be fed
        as model input to the sentiment analysis model 
    */
    let default_embedding_dim: usize = 1024;

    let mut aggregated_embeddings = if let Some(first_token) = tokens.get(0) {
        if let Some(embedding) = embeddings.get(first_token) {
            // succes - token existed and there was an embedding for it
            Array1::<f64>::zeros(embedding.dim())
        } else {
            // otherwise, we use a default embedding dim
            Array1::<f64>::zeros(default_embedding_dim)
        }
    } else {
        // didn't find token, also use default
        Array1::<f64>::zeros(default_embedding_dim)
    };

    let mut count: i32 = 0; // will divide aggregated_embeddings by total count to get average

    for token in tokens {
        if let Some(embedding) = embeddings.get(&token) {
            // embedding found, add it to our aggregation
            aggregated_embeddings = aggregated_embeddings + embedding;
            count += 1;
        } 
    }

    if count > 0 {
        aggregated_embeddings /= count as f64;
    }

    return aggregated_embeddings

}