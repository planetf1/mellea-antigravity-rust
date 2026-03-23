use mellea::backends::OllamaBackend;
use mellea::{MelleaSession, RejectionSamplingConfig};
use std::error::Error;

/// This example demonstrates the primitive underlying Mellea's `@generative` feature in Python.
/// In Rust, we wrap the LLM call in a strongly-typed native Rust function ("Generative Slot").
/// The rest of the application never needs to know that an LLM is fulfilling the logic!

async fn classify_sentiment(m: &MelleaSession<OllamaBackend>, text: &str) -> std::result::Result<String, Box<dyn Error>> {
    let email = m.instruct(&format!("Classify the sentiment of this review: '{}'", text))
        .with_requirement("The response MUST be exactly one word: 'positive', 'negative', or 'neutral'. Absolutely no other text.")
        .with_strategy(RejectionSamplingConfig::new().loop_budget(4)) // Give it a high budget since parsing is strict
        .execute()
        .await?;

    Ok(email.content.trim().to_lowercase())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Demonstrating Generative Functions ---");
    
    let backend = OllamaBackend::new("granite4:micro");
    let m = MelleaSession::new(backend);

    let review = "The app keeps crashing when I upload pictures. Disappointing.";
    println!("Review: '{}'", review);
    
    // We call the LLM like it's just a normal Rust function!
    // The IVR logic inside ensures the result matches our Enum/String constraints perfectly.
    let sentiment = classify_sentiment(&m, review).await?;
    
    println!("Parsed Sentiment Variable: {}", sentiment);

    // Now we can use native rust conditionals fearlessly:
    if sentiment == "negative" {
        println!("ACTION: Routing to Customer Support queue!");
    } else {
        println!("ACTION: Filing away in analytics database.");
    }

    Ok(())
}
