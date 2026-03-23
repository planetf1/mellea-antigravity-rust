# Mellea Rust

Mellea is a library for writing generative programs.
Generative programming replaces flaky agents and brittle prompts with structured, maintainable, robust, and efficient AI workflows.

This repository contains the **Proof of Concept (PoC)** of porting the `mellea` core concepts into Rust, emphasizing strong static typing, elegant zero-cost abstractions, and fearlessly concurrent execution loops.

## Features

- **Standardized Instruction Pattern**: Use the `Instruct-Validate-Repair` loop directly in Rust using idiomatic builder patterns.
- **Strictly Typed Execution**: Combine with `serde` to strictly enforce LLM JSON payloads.
- **Rejection Sampling Built-In**: Automatically configure a validation loop budget. If a constraint fails, `mellea` will transparently repair the instruction up to the budget limits without brittle while-loops in your business logic.
- **Modular Backends and Verifiers**: Easy `Trait` implementation for `ModelBackend` (like `OllamaBackend`) and `GenerativeVerifier`.

## Getting Started (Before vs After)

To see why Mellea brings value, compare how LLMs are typically integrated in Rust vs how they are integrated with `mellea`.

Run the native brittle approach:
```bash
cargo run --example before_mellea
```

Run the robust Gen-Prog boundary using Mellea's Instruct-Validate-Repair loop:
```bash
cargo run --example after_mellea
```

See how Mellea powers transparent Generative Functions (like python's `@generative`):
```bash
cargo run --example generative_functions
```

See how Mellea forces Native Rust Structs (`serde` extraction):
```bash
cargo run --example structured_data
```

## Setup

This PoC uses Ollama as the default backend for inference.
1. Install [Ollama](https://ollama.com).
2. Pull the model matching our examples: `ollama pull granite4:micro`.
3. Run the examples!
