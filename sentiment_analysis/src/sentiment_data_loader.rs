// Loads in .csv file containing labeled sentiment data
// We are using Sentiment140 dataset from Kaggle
// https://www.kaggle.com/kazanova/sentiment140

use serde::Deserialize;
use csv::Reader;
use std::error::Error;

// Each column in csv file corresponds to a field in the struct
// Use Debug to format print the struct
// Deserialize to convert the csv file into a struct
// we need to implement Deserialize to our struct as it provides the instructions
// for converting our csv file into the struct format specified below
#[derive(Debug, Deserialize)]
pub struct SentimentData {
    target: String,
    id: String,
    date: String,
    flag: String,
    user: String,
    text: String,
}

pub fn read_csv(file_path: &str) -> Result<Vec<SentimentData>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut sentiment_data = Vec::new();

    // deserialize loops through each row in the csv file - returns an iterator
    // we then parse the row with adherence to the structure of SentimentData
    // rdr.deserialize() uses the Deserialize implementation of our struct to
    // understand how to convert each row of the csv file into an instance of the struct
    for result in rdr.deserialize() {
        let record: SentimentData = result?;
        sentiment_data.push(record);
    }

    Ok(sentiment_data)
}