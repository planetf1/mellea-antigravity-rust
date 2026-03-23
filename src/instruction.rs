use crate::backend::{GenerationRequest, GenerationResponse, ModelBackend};
use crate::error::{MelleaError, Result};
use crate::strategy::RejectionSamplingConfig;
use crate::verifier::{GenerativeVerifier, LLMAsAJudgeVerifier};
use std::sync::Arc;

/// Builder pattern for configuring and executing an LLM instruction.
pub struct InstructionBuilder<B: ModelBackend> {
    backend: Arc<B>,
    prompt: String,
    requirements: Vec<String>,
    strategy: RejectionSamplingConfig,
}

impl<B: ModelBackend + Send + Sync + 'static> InstructionBuilder<B> {
    pub(crate) fn new(backend: Arc<B>, prompt: String) -> Self {
        Self {
            backend,
            prompt,
            requirements: Vec::new(),
            strategy: RejectionSamplingConfig::default(),
        }
    }

    /// Add a strict requirement that the generated output must meet.
    pub fn with_requirement(mut self, req: &str) -> Self {
        self.requirements.push(req.to_string());
        self
    }

    /// Set the retry strategy configuration (e.g. RejectionSamplingConfig).
    pub fn with_strategy(mut self, strategy: RejectionSamplingConfig) -> Self {
        self.strategy = strategy;
        self
    }

    /// Execute the instruction asynchronously, performing the Validate-Repair loop implicitly.
    pub async fn execute(self) -> Result<GenerationResponse> {
        let budget = self.strategy.loop_budget;
        let mut loop_count = 0;

        let verifier = LLMAsAJudgeVerifier::new(self.backend.clone());

        while loop_count < budget {
            let mut final_prompt = self.prompt.clone();
            if !self.requirements.is_empty() {
                final_prompt.push_str("\n\nPlease ensure your response firmly meets the following requirements:\n");
                for r in &self.requirements {
                    final_prompt.push_str(&format!("- {}\n", r));
                }
            }

            let req = GenerationRequest {
                prompt: final_prompt,
                model: self.backend.default_model(),
            };

            // Call model
            let response = self.backend.generate(req).await?;

            // Validation step
            let mut all_passed = true;
            for requirement in &self.requirements {
                let passed = verifier.verify(&response.content, requirement).await?;
                if !passed {
                    all_passed = false;
                    break;
                }
            }

            if all_passed {
                return Ok(response);
            }

            // Repair step (simple rejection and retry in PoC MVP)
            loop_count += 1;
        }

        Err(MelleaError::ExhaustedBudgetError(budget))
    }
}
