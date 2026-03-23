use mellea::backends::OllamaBackend;
use mellea::{MelleaSession, RejectionSamplingConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Running With Mellea ---");
    
    // 1. Instantiate our backend pointing to a local inference model
    let backend = OllamaBackend::new("granite4:micro");
    
    // 2. Wrap it in a Generative Programming Session
    let m = MelleaSession::new(backend);

    println!("Executing Instruction. Mellea's Instruct-Validate-Repair loop is handling compliance...");
    
    // REALISTIC SCENARIO: Customer Support Email.
    // Business Rule: We must apologize, but we DO NOT offer refunds or discounts for late deliveries.
    
    // 3. The Rejection Sampling strategy evaluates the requirements automatically using an LLM-as-a-judge
    //    and loops up to 3 times entirely transparently to the developer to guarantee compliance.
    let email = m.instruct("Draft an email to a customer whose delivery is 3 days late. Apologize profusely.")
        .with_requirement("The email must sound empathetic and professional.")
        .with_requirement("The email MUST NOT offer any refunds, discounts, coupons, or free gifts. This is a strict company policy.")
        .with_strategy(RejectionSamplingConfig::new().loop_budget(3))
        .execute()
        .await?;

    println!("\nVERIFIED EMAIL OUTPUT:\n{}\n", email.content);

    Ok(())
}
