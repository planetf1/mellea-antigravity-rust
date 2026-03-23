use crate::backend::ModelBackend;
use crate::instruction::InstructionBuilder;
use std::sync::Arc;

/// MelleaSession contains the context required to run instructions, including the active ModelBackend.
pub struct MelleaSession<B: ModelBackend> {
    backend: Arc<B>,
}

impl<B: ModelBackend + Send + Sync + 'static> MelleaSession<B> {
    /// Create a new session attached to the provided backend.
    pub fn new(backend: B) -> Self {
        Self {
            backend: Arc::new(backend),
        }
    }

    /// Start building an instruction to query the LLM
    pub fn instruct(&self, prompt: &str) -> InstructionBuilder<B> {
        InstructionBuilder::new(self.backend.clone(), prompt.to_string())
    }
}
