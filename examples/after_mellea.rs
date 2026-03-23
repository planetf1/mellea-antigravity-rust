use mellea::backends::OllamaBackend;
use mellea::{MelleaSession, RejectionSamplingConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Running With Mellea ---");
    
    // 1. Instantiate our backend pointing to a local inference model
    let backend = OllamaBackend::new("mistral:latest");
    
    // 2. Wrap it in a Generative Programming Session
    let m = MelleaSession::new(backend);

    println!("Executing Instruction. Mellea's Instruct-Validate-Repair loop is handling compliance...");
    
    // 3. The Rejection Sampling strategy evaluates the requirements automatically using an LLM-as-a-judge
    //    and loops up to 3 times entirely transparently to the developer to guarantee compliance.
    let email = m.instruct("Write an email to invite all interns to the office party.")
        .with_requirement("be formal")
        .with_requirement("Use 'Dear interns,' as greeting.")
        .with_strategy(RejectionSamplingConfig::new().loop_budget(3))
        .execute()
        .await?;

    println!("\nVERIFIED EMAIL OUTPUT:\n{}\n", email.content);

    Ok(())
}
