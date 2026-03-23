use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ollama = Ollama::default();
    
    // We embed the instruction manually inside the prompt text.
    let prompt = "Write an email to invite all interns to the office party. MUST start with 'Dear interns,' and MUST be formal.";
    
    println!("--- Running Without Mellea ---");
    println!("Sending request to Ollama using raw ollama-rs...");
    
    // WITHOUT MELLEA: Just sending a raw string, hoping it listens.
    let req = GenerationRequest::new("mistral:latest".to_string(), prompt.to_string());
    let res = ollama.generate(req).await?;
    
    let content_lower = res.response.to_lowercase();

    // Manual rudimentary string check to demonstrate pain
    if content_lower.contains("dear interns") && !content_lower.contains("hey guys") {
        println!("\nSuccess. The model followed the prompt manually.\nResult:\n{}\n", res.response);
    } else {
        println!("\nFailed! The model ignored our strict instructions:\n{}\n", res.response);
        println!("Note: If this were production, the developer would now have to write their own manual while-loop with their own validation LLM prompt to automatically self-correct this.");
    }
    
    Ok(())
}
