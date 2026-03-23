# Mellea-Rust PoC: AI Development Retrospective

## What Was Easy
1. **Mapping Mellea's Concepts to Rust**: Rust's architecture is uniquely suited for the Mellea pattern. Concepts like `GenerativeVerifier` and `ModelBackend` slotted perfectly into Rust `Traits`. Furthermore, mapping Generative loops into native Rust `Result<T, E>` types made error handling far more elegant than Python's dynamic exception bubbling.
2. **Demonstrating the Value-Add**: Because extracting structured JSON from an LLM in Rust usually involves brittle string matching and frequent parser panics, it was incredibly easy to prove Mellea's worth. `serde_json::from_str` combined with Mellea's `Instruct-Validate-Repair` loop visually sells itself.

## What Was Hard
1. **Making the "Before" Example Fail Reliably**: LLMs are stochastic. Initially, I drafted a `before_mellea` example that said: *"Write an email... MUST start with 'Dear Interns'"*. Modern LLMs are actually quite good at following basic formatting commands on the first try, which meant the script natively succeeded without Mellea's help! It took a few iterations to find a constraint that local LLMs routinely fail at (e.g. strict JSON mapping or resisting negative constraints like *"Do NOT offer a refund"*) in order to truly prove Mellea's repair mechanism.
2. **Hardware/Environment Assumptions**: The original prompt suggested testing against `mistral`, but the local machine didn't have it pulled. We had to pivot mid-execution to map the examples to IBM Granite.

## What I Would Tell the Original Requester
> *"Excellent job centering the entire request around the **Core Value Proposition**."*

Instead of just asking for a 1:1 literal translation of the Python codebase into Rust, you explicitly asked to build **Before/After code examples** that sell the idea to developers. This was a brilliant piece of instruction. It forced the architecture of the PoC to be laser-focused on Developer Experience (DX) and highlighting Mellea's `Instruct-Validate-Repair` strengths. 

## How the Process Could Be Better Next Time Around
1. **Provide Environment Constraints Upfront**: If you want end-to-end execution, explicitly listing the available local resources (e.g., *"Use `granite4:micro` running on local `ollama`"*) in the `PROMPT.md` prevents unnecessary loops troubleshooting missing dependencies.
2. **Define the Business Logic of the Examples**: Rather than leaving the sample app open-ended, outlining a specific, tricky generative scenario upfront (e.g., *"Build an example that parses a messy bio into a strict UserProfile struct"*) would allow the execution phase to immediately target the most persuasive LLM use-cases.

## What Questions Do You Have?
1. Does the developer experience of the Mellea-Rust builder API (`m.instruct().with_requirement().execute().await?`) adequately capture the design aesthetic of the Python library?
2. The current `LLMAsAJudge` verifier embeds the context into a hardcoded prompt string. Should the verifier implement customizable prompt-templating moving forward for stricter business control?
3. What is the intended relationship between Mellea and `async` streaming? Will customers want to stream validation loop chunks back to a UI, or should the library exclusively buffer outputs until the `Instruct-Validate-Repair` loop finalized?

## What Would You Do Next?
1. **Procedural Macros (`#[mellea::generative]`)**: This is the absolute highest priority. I would create a `mellea-macros` crate to implement the `@generative` decorator experience natively at compile-time in Rust, perfectly mimicking Python's DX by hiding the `MelleaSession` instantiation from standard function boundaries.
2. **Enterprise Backends**: I would write additional `ModelBackend` trait implementations relying on `reqwest` to support `WatsonxBackend`, `vLLMBackend`, and `OpenAIBackend`.
3. **Advanced Sampling Strategies**: Now that `RejectionSampling` works natively, I would implement more complex strategies like *Majority Voting* or *Self-Refine* to further guarantee constraint compliance.
