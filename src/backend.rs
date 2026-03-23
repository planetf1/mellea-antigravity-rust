use crate::error::Result;
use std::future::Future;

/// Parameters for generation requests sent to a ModelBackend
#[derive(Debug, Clone)]
pub struct GenerationRequest {
    pub prompt: String,
    pub model: String,
}

/// Content returned from a ModelBackend
#[derive(Debug, Clone)]
pub struct GenerationResponse {
    pub content: String,
}

/// The core trait that all Mellea model runners (like Ollama, vLLM, etc.) must implement.
pub trait ModelBackend: Send + Sync {
    /// Send a parameterized request to the backend and asynchronously await the generation response.
    fn generate(&self, req: GenerationRequest) -> impl Future<Output = Result<GenerationResponse>> + Send;
    
    /// Return the default model identifier this backend utilizes when unspecified.
    fn default_model(&self) -> String;
}
