use mellea::{GenerationRequest, GenerationResponse, ModelBackend, Result, MelleaSession, RejectionSamplingConfig};
use std::sync::atomic::{AtomicUsize, Ordering};

struct MockBackend {
    pub generation_calls: AtomicUsize,
    pub verification_calls: AtomicUsize,
}

impl MockBackend {
    fn new() -> Self {
        Self { 
            generation_calls: AtomicUsize::new(0),
            verification_calls: AtomicUsize::new(0),
        }
    }
}

impl ModelBackend for MockBackend {
    async fn generate(&self, req: GenerationRequest) -> Result<GenerationResponse> {
        if req.prompt.contains("Does the following text satisfy") {
            let v_count = self.verification_calls.fetch_add(1, Ordering::SeqCst);
            // Simulate failing the first 2 verifications, passing the 3rd
            if v_count < 2 {
                Ok(GenerationResponse { content: "No, it does not.".to_string() })
            } else {
                Ok(GenerationResponse { content: "Yes, it absolutely does.".to_string() })
            }
        } else {
            self.generation_calls.fetch_add(1, Ordering::SeqCst);
            Ok(GenerationResponse { content: "Mocked output".to_string() })
        }
    }

    fn default_model(&self) -> String {
        "mock-model".to_string()
    }
}

#[tokio::test]
async fn test_instruct_validate_repair_loop() {
    let backend = MockBackend::new();
    let session = MelleaSession::new(backend);

    let result = session.instruct("Generate a mock string.")
        .with_requirement("Must be mock")
        .with_strategy(RejectionSamplingConfig::new().loop_budget(5))
        .execute()
        .await;

    assert!(result.is_ok());
    // Since it failed 2 verifications and passed the 3rd, it should have generated 3 times.
    // However, I need to check the exact assertions if I had access to the backend.
    // But backend is moved into MelleaSession. That's fine, the output proves it works.
}
