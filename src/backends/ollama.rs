use crate::backend::{GenerationRequest, GenerationResponse, ModelBackend};
use crate::error::{MelleaError, Result};
use ollama_rs::generation::completion::request::GenerationRequest as OllamaGenReq;
use ollama_rs::Ollama;

pub struct OllamaBackend {
    client: Ollama,
    default_model_name: String,
}

impl OllamaBackend {
    /// Create a new Ollama backend using the default host (http://localhost:11434)
    pub fn new(model_name: &str) -> Self {
        Self {
            client: Ollama::default(),
            default_model_name: model_name.to_string(),
        }
    }
}

impl ModelBackend for OllamaBackend {
    async fn generate(&self, req: GenerationRequest) -> Result<GenerationResponse> {
        let ollama_req = OllamaGenReq::new(req.model, req.prompt);
        let response = self
            .client
            .generate(ollama_req)
            .await
            .map_err(|e| MelleaError::GenerationError(e.to_string()))?;

        Ok(GenerationResponse {
            content: response.response,
        })
    }

    fn default_model(&self) -> String {
        self.default_model_name.clone()
    }
}
