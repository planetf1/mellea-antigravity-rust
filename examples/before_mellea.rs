use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ollama = Ollama::default();
    
    // REALISTIC SCENARIO: Customer Support Email.
    // Business Rule: We must apologize, but we DO NOT offer refunds or discounts for late deliveries.
    let prompt = "Draft an email to a customer whose delivery is 3 days late. Apologize profusely. Do NOT offer any refunds, coupons, or free gifts under any circumstances.";
    
    println!("--- Running Without Mellea ---");
    println!("Sending request to Ollama using raw ollama-rs...");
    
    // WITHOUT MELLEA: Just sending a raw string, hoping the LLM respects the negative constraint.
    let req = GenerationRequest::new("granite4:micro".to_string(), prompt.to_string());
    let res = ollama.generate(req).await?;
    
    let content_lower = res.response.to_lowercase();

    // In a real application, business constraints are critical. 
    // LLMs famously struggle with negative constraints ("Do NOT offer refunds").
    if content_lower.contains("refund") || content_lower.contains("discount") || content_lower.contains("coupon") {
        println!("\n[FATAL BUSINESS ERROR]: The model ignored our strict negative constraint and cost the company money by offering a refund/discount:\n\n{}\n", res.response);
        println!("Note: Without Mellea, this rogue response goes straight to the customer. To fix this, developers usually have to hand-roll custom orchestration, LLM-judges, and while-loops.");
    } else {
        println!("\n[Success]: The model followed the prompt natively.\nResult:\n{}\n", res.response);
    }
    
    Ok(())
}
