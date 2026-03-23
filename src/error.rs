use thiserror::Error;

#[derive(Error, Debug)]
pub enum MelleaError {
    #[error("Backend configuration error: {0}")]
    ConfigError(String),

    #[error("Failed to connect to backend: {0}")]
    BackendConnectionError(String),

    #[error("Exhausted budget of {0} loops without satisfying requirements")]
    ExhaustedBudgetError(usize),

    #[error("Model generation error: {0}")]
    GenerationError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, MelleaError>;
