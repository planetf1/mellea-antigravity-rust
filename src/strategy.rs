/// Configuration for the Instruct-Validate-Repair retry logic.
#[derive(Debug, Clone)]
pub struct RejectionSamplingConfig {
    pub loop_budget: usize,
}

impl RejectionSamplingConfig {
    /// Creates a new configuration with a default loop budget of 3.
    pub fn new() -> Self {
        Self { loop_budget: 3 }
    }

    /// Set the maximum number of attempts the strategy will make to satisfy requirements.
    pub fn loop_budget(mut self, budget: usize) -> Self {
        self.loop_budget = budget;
        self
    }
}

impl Default for RejectionSamplingConfig {
    fn default() -> Self {
        Self::new()
    }
}
