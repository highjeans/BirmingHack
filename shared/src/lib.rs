use std::iter::zip;

use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

fn dissimilar() -> Option<(f32, f32)> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()
        .ok()?;
    let sentences = [
        "this is an example sentence",
        "each sentence is converted",
        "this is a dissimilar, different, weird version",
    ];
    let output = model.encode(&sentences).ok()?;
    Some((
        cosine_similarity(&output[0], &output[1]),
        cosine_similarity(&output[1], &output[2]),
    ))
}

fn dot_prod(v: &Vec<f32>, u: &Vec<f32>) -> f32 {
    zip(v, u).map(|(v1, u1)| v1 * u1).sum()
}

fn cosine_similarity(v: &Vec<f32>, u: &Vec<f32>) -> f32 {
    let dp = dot_prod(v, u);
    let norm_v: f32 = v.iter().map(|v| v.powi(2)).sum();
    let norm_u: f32 = u.iter().map(|u| u.powi(2)).sum();
    dp / norm_v.sqrt() * norm_u.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (sim, disim) = dissimilar().unwrap();
        assert!(sim > disim)
    }
}
