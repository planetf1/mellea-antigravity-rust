# Mellea Rust

Mellea is a library for writing robust generative programs. It replaces flaky agents and brittle string prompts with structured, maintainable, and type-safe AI workflows.

This repository contains the **Proof of Concept (PoC)** to bring Python's `mellea` core concepts into the Rust ecosystem. By combining Mellea with Rust's excellent traits, async model (`tokio`), and serialization (`serde`), we unlock zero-cost abstractions for fearlessly enforcing programmatic restraints on Non-Deterministic Computing (LLMs).

## Core Value Proposition

When using raw wrappers (like `ollama-rs` or `reqwest`), negative constraints (e.g. "Do NOT offer refunds") and formatting rules (e.g. "Return ONLY JSON") frequently fail in production. Resolving this forces developers to write ugly, hand-rolled string validation and `while-loops` polluting business logic.

Mellea solves this elegantly with the **Instruct-Validate-Repair (IVR)** pattern.

### 0. The Native Rust Struggle (Without Mellea)
When using a native backend directly, you just send strings and pray the LLM respects your business rules. If it ignores a negative constraint (e.g. "Do NOT offer refunds"), the bad text goes straight into your application logic unless you build a bespoke while-loop validation engine yourself.

**[See `examples/before_mellea.rs` for the full script](examples/before_mellea.rs)**
```rust
let prompt = "Draft an email to a customer... Do NOT offer any refunds or coupons.";
let req = GenerationRequest::new("granite4:micro".to_string(), prompt.to_string());
let res = ollama.generate(req).await?;

// Hand-rolled, brittle string-checking polluting your business logic
let content_lower = res.response.to_lowercase();
if content_lower.contains("refund") || content_lower.contains("coupon") {
    println!("[FATAL BUSINESS ERROR]: The model improperly offered a refund!");
    // You now have to write your own orchestration loop to fix this...
}
```

### 1. The Power of `MelleaSession` (Instruct-Validate-Repair)
Using the elegant builder API, developers can define strict Requirements. If the LLM generates output violating those requirements, Mellea's internal `RejectionSamplingConfig` loop intercepts the failure, utilizes an `LLMAsAJudgeVerifier`, and automatically repairs the generation invisibly.

**[See `examples/after_mellea.rs` for the full script](examples/after_mellea.rs)**
```rust
let email = m.instruct("Draft an email to a customer whose delivery is 3 days late. Apologize profusely.")
    .with_requirement("The email must sound empathetic and professional.")
    .with_requirement("The email MUST NOT offer any refunds, discounts, coupons, or free gifts.")
    .with_strategy(RejectionSamplingConfig::new().loop_budget(3))
    .execute()
    .await?;
```
*(Compare this against the brittle, native approach in [`examples/before_mellea.rs`](examples/before_mellea.rs))*

### 2. Generative Slots / Functions 
Mellea lets you abstract the Prompt engineering entirely away from the wider application, creating "Generative Slots". Your application just calls standard, strictly-typed `async fn`s.

**[See `examples/generative_functions.rs` for the full script](examples/generative_functions.rs)**
```rust
async fn classify_sentiment(m: &MelleaSession<OllamaBackend>, text: &str) -> Result<String, Box<dyn Error>> {
    let response = m.instruct(&format!("Classify the sentiment of this review: '{}'", text))
        .with_requirement("The response MUST be exactly one word: 'positive', 'negative', or 'neutral'. Absolutely no other text.")
        .with_strategy(RejectionSamplingConfig::new().loop_budget(4)) 
        .execute()
        .await?;

    Ok(response.content.trim().to_lowercase())
}
```

### 3. Fearless Structured Data Deserialization
The holy grail of LLMs in Rust is mapping unstructured strings into native `#[derive(Deserialize)]` Structs. Mellea's IVR loop prevents parser panics by trapping the LLM into generating pure, syntactically flawless JSON *before* it hits the `serde_json::from_str` boundary.

**[See `examples/structured_data.rs` for the full script](examples/structured_data.rs)**
```rust
#[derive(Deserialize, Debug)]
struct UserProfile {
    pub username: String,
    pub age: u8,
    pub hobbies: Vec<String>,
}

// ... 

let response = m.instruct(&prompt)
    .with_requirement("The response MUST be ONLY a single valid JSON object. No conversational text. No markdown blocks.")
    .with_requirement("The response MUST strictly start with '{' and end with '}'.")
    .with_strategy(RejectionSamplingConfig::new().loop_budget(4))
    .execute()
    .await?;

// Fearlessly decode because Mellea guaranteed the JSON structure!
let profile: UserProfile = serde_json::from_str(&response.content.trim())?;
```

---

## Setup & Running Examples

This PoC uses Ollama as the default backend for inference.

1. Install [Ollama](https://ollama.com).
2. Pull the model matching our examples: `ollama pull granite4:micro`.
3. Read the terminal output of our core examples!

```bash
cargo run --example before_mellea
cargo run --example after_mellea
cargo run --example generative_functions
cargo run --example structured_data
```

## Additional Documentation

- **[Project Retrospective](RETROSPECTIVE.md)**: A breakdown of the Proof of Concept development, feedback to the original request prompt, and a roadmap for future Mellea-Rust features.
