use std::iter::zip;

use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};

fn dissimilar() -> Option<(f32, f32)> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()
        .ok()?;
    let sentences = [
        "When seventeen-year-old Bella Swan moves to the gloomy town of Forks, Washington, she doesn’t expect much—until she meets Edward Cullen, a mysterious and captivating student at her high school. His piercing gaze, strange behavior, and elusive nature quickly intrigue her, but Bella’s curiosity uncovers a secret far more dangerous than she could ever imagine. Edward is a vampire, and as their forbidden love grows deeper, Bella finds herself caught in a world of supernatural danger, where every moment with him is a risk. Torn between love and survival, Bella must navigate the perils of her heart’s desire while facing the dangers that come with falling in love with a creature of the night.",
        "James Henry Trotter’s life takes a magical turn when he’s given a mysterious gift: a giant, glowing peach. After a series of incredible events, James finds himself inside the peach, where he meets a group of oversized, talking insects—each with their own quirks and personalities. Together, they embark on a fantastical journey across the ocean, facing terrifying creatures and thrilling adventures along the way. As James and his new friends overcome their fears and grow stronger, they discover that with courage, friendship, and a bit of imagination, anything is possible.",
        "A must-have guide for software developers, Design Patterns introduces 23 fundamental design patterns that have revolutionized object-oriented programming. This groundbreaking book provides reusable solutions to common software design problems, helping developers create flexible, maintainable, and scalable systems. The authors—often referred to as the \"Gang of Four\"—discuss proven best practices and offer detailed examples for implementing patterns such as Singleton, Observer, and Factory Method. Whether you're a seasoned developer or just starting out, this essential reference will deepen your understanding of object-oriented design and empower you to build more efficient, elegant software systems",
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
