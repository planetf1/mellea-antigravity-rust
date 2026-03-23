use mellea::backends::OllamaBackend;
use mellea::{MelleaSession, RejectionSamplingConfig};
use serde::Deserialize;
use std::error::Error;

/// The greatest value-add for Rust in Generative Programming is mapping fuzzy text 
/// safely into strictly-typed `serde` struct representations.

#[derive(Deserialize, Debug)]
struct UserProfile {
    pub username: String,
    pub age: u8,
    pub hobbies: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Demonstrating Structured LLM Extraction ---");
    
    let backend = OllamaBackend::new("granite4:micro");
    let m = MelleaSession::new(backend);

    let messy_input = "Hi! I'm xX_dragonSlayer_Xx. I literally just turned 24 yesterday. When I'm not coding in Rust, I'm usually rock climbing or playing chess.";
    println!("Raw Unstructured Input:\n'{}'\n", messy_input);
    
    // We bind the LLM using Instruct-Validate-Repair to force the JSON compliance.
    // If it outputs conversational text (e.g. "Here is the JSON..."), the loop catches it
    // and repairs it before it ever reaches our `serde_json::from_str` boundary!
    let prompt = format!(
        "Extract the user profile from this text: '{}'.\nOutput as exclusively a JSON object with keys: 'username' (string), 'age' (integer), and 'hobbies' (array of strings).", 
        messy_input
    );

    let response = m.instruct(&prompt)
        .with_requirement("The response MUST be ONLY a single valid JSON object. No conversational text. No markdown blocks.")
        .with_requirement("The response MUST strictly start with '{' and end with '}'.")
        .with_strategy(RejectionSamplingConfig::new().loop_budget(4)) // Give a high budget just in case Granite is chatty
        .execute()
        .await?;

    println!("Safely constrained unstructured generation to pure JSON...");
    
    // Fearless Rust deserialization because Mellea's generation boundaries protected us.
    let profile: UserProfile = serde_json::from_str(&response.content.trim())?;
    
    println!("\nSuccessfully Extracted NATIVE RUST Struct Target:\n{:#?}\n", profile);

    Ok(())
}
