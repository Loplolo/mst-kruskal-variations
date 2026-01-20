use std::fmt;

#[derive(Debug, Clone)]
pub enum GraphError {
    InvalidProbability(f64),
    InvalidCostRange { min: usize, max: usize },
    EmptyInput,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::InvalidProbability(p) => {
                write!(f, "Probability must be between 0.0 and 1.0, got {}", p)
            }
            GraphError::InvalidCostRange { min, max } => {
                write!(f, "Invalid cost range: min ({}) > max ({})", min, max)
            }
            GraphError::EmptyInput => write!(f, "Input collection cannot be empty"),
        }
    }
}

impl std::error::Error for GraphError {}
