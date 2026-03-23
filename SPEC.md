# Mellea-Rust: Proof of Concept Specification

## 1. Reason for the PoC

The primary reason for writing `mellea-rust` is to bring the powerful concepts of **Generative Programming**—currently realized in the Python `mellea` library—into the Rust ecosystem. 

Generative programming treats Large Language Models (LLMs) not just as chat agents, but as non-deterministic compute blocks that can be constrained, verified, and repaired. The `mellea-rust` PoC aims to prove that these patterns (like Instruct-Validate-Repair) are not only feasible in Rust, but that they naturally benefit from Rust's strong static type system, memory safety, and elegant error-handling features.

## 2. Benefits of Mellea-Rust

Using Mellea in Rust provides unique advantages compared to dynamic languages:
- **Type-Safe Structured Output:** By deeply integrating with `serde` and `schemars`, `mellea-rust` guarantees that LLM outputs deserialize correctly into Rust structs or it triggers the automatic Repair loop.
- **Zero-Cost Abstractions:** Extensible traits (`Verifier`, `ModelBackend`) allow users to swap out validation mechanisms or model providers without severe runtime overhead.
- **Fearless Concurrency:** In scenarios requiring high-throughput generative evaluations, Rust's `tokio`-based async model handles numerous concurrent LLM API calls and validation steps efficiently.
- **Robust Error Handling:** Missing or malfunctioning LLM calls are explicitly boxed in manageable `Result` types (e.g., `MelleaError`), forcing developers to handle failures gracefully.

## 3. Comparison with Existing Rust Ecosystem

The Rust ecosystem contains several LLM-related crates, but most operate at different abstraction layers:
- **`async-openai` / `ollama-rs`:** These are *raw API clients*. They handle the HTTP requests but leave the orchestration, parsing, prompting strategies, and robust retry logic entirely up to the developer.
- **`candle` / `llm`:** These are *inference engines* meant for running models locally (like `llama.cpp` equivalents), not orchestration libraries.
- **`rig-rs` / `langchain-rust`:** These focus more on RAG (Retrieval-Augmented Generation), Agentic workflows, and chaining. 

**Mellea-Rust** fills the missing gap: **Generative Programming primitives.** It specifically provides loops designed to wrap inference inside strong validation contracts (Instruct-Validate-Repair), treating model calls as robust functions rather than unpredictable chat blocks.

## 4. Code Example: Before and After

To understand the core value proposition, consider what a developer has to do *without* Mellea to ensure an LLM respects a prompt requirement.

### **Before Mellea (Using raw `ollama-rs`)**
Without Mellea, ensuring an LLM response meets constraints requires manual retry loops, manual validation, and manual error accumulation:

```rust
// A brittle, manual approach
let mut retries = 0;
let mut response_text = String::new();

while retries < 3 {
    let res = ollama.generate(GenerationRequest::new("mistral", "Write a formal email. MUST start with 'Dear Interns',")).await?;
    response_text = res.response;
    
    // Manual validation step
    if response_text.starts_with("Dear Interns,") {
        break; // Success
    } else {
        retries += 1;
        // The context of WHY it failed isn't easily fed back in the next prompt
        // unless the developer manually crafts a "Repair" prompt here.
    }
}

if !response_text.starts_with("Dear Interns,") {
    panic!("Model failed to follow instructions after 3 retries");
}
```

### **After Mellea**
With `mellea-rust`, this orchestration is embedded in the library using the **Instruct-Validate-Repair** pattern, utilizing elegant Rust builder idioms:

```rust
// An elegant, strongly-typed Generative Programming approach
use mellea::{MelleaSession, RejectionSamplingConfig};
use mellea::backends::OllamaBackend;

let m = MelleaSession::new(OllamaBackend::new("mistral"));

let email = m.instruct("Write an email to invite all interns to the office party.")
             .with_requirement("be formal")
             .with_requirement("Use 'Dear interns' as greeting.")
             .with_strategy(RejectionSamplingConfig::new().loop_budget(3))
             .execute()
             .await?;

println!("Verified Email: {}", email.content);
```

## 5. Adopting Rust Idioms over Direct Python Ports

Instead of porting the Python codebase line-by-line, `mellea-rust` embraces Rust paradigms:
- **`Builder` Pattern:** Used for constructing instructions (e.g., `.with_requirement()`, `.with_strategy()`) instead of Python's massive kwarg functions.
- **Traits for Interfaces:** The concept of `backend` and `verifier` will be backed by Rust `Trait` functionality (`trait ModelBackend`, `trait GenerativeVerifier`), allowing users to easily struct-impl their own components.
- **Macros (Future functionality):** While Python uses the `@generative` decorator to convert standard functions into LLM calls, Rust can utilize *procedural macros* (e.g., `#[mellea::generative]`) to achieve the exact same DX at compile time.
- **`thiserror` Crate:** Proper nested error types rather than dynamic exceptions, so failures during the Validate-Repair loop are introspectable natively using `match`.

## 6. Testing, Docs, and Delivery
- **Documentation:** Inline rustdocs (`///`) will be heavily utilized so `cargo doc --open` produces beautiful, standard documentation.
- **Testing:** Unit tests for traits/parsers, and integration tests ensuring the validation loops correctly identify and reject synthetic bad model answers. We will also utilize an `examples/` directory for the `before/after` scripts that can be executed directly via `cargo run --example tutorial`.
- **LLM Support:** Will rely primarily on `Ollama` running locally for simple testing without API keys, defaulting to a lightweight model (e.g., `mistral` or `granite`).
