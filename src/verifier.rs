use crate::error::Result;
use crate::backend::{ModelBackend, GenerationRequest};
use std::future::Future;
use std::sync::Arc;

/// A Verifier ensures that generated content conforms to specific requirements.
pub trait GenerativeVerifier: Send + Sync {
    /// Check whether `content` satisfies the strict `requirement`.
    fn verify(&self, content: &str, requirement: &str) -> impl Future<Output = Result<bool>> + Send;
}

/// Uses a provided ModelBackend to judge whether text meets requirements.
pub struct LLMAsAJudgeVerifier<B: ModelBackend> {
    backend: Arc<B>,
}

impl<B: ModelBackend> LLMAsAJudgeVerifier<B> {
    /// Create a new verifier that depends on the specified backend.
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }
}

impl<B: ModelBackend + Send + Sync> GenerativeVerifier for LLMAsAJudgeVerifier<B> {
    async fn verify(&self, content: &str, requirement: &str) -> Result<bool> {
        let prompt = format!(
            "You are a strict verifier. Does the following text satisfy this requirement: '{}'?\n\nText: '{}'\n\nAnswer only 'Yes' or 'No'.",
            requirement, content
        );
        let req = GenerationRequest {
            prompt,
            model: self.backend.default_model(),
        };
        let response = self.backend.generate(req).await?;
        let lower = response.content.to_lowercase();
        // Strict yes check
        Ok(lower.contains("yes") && !lower.contains("no"))
    }
}
