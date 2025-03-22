use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use serde::{Deserialize};

fn example() -> Option<(String)> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()
        .ok()?;

    let sentences = ["this is an example sentence", "each sentence is converted"];

    let output = model.encode(&sentences).ok()?;
    Some(format!("{output:?}"))
}

#[derive(Deserialize)]
pub struct SignupData {
    pub username: String,
    pub password: String,
    pub fullname: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn it_works() {
    //    assert_eq!(example().unwrap_or_default(), String::from("69"));
    //}
}
