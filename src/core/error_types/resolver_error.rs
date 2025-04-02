#[derive(Debug, Clone)]
pub enum ResolverError {
    TopLevelReturn,
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolverError::TopLevelReturn => write!(f, "[RESOLVER]: Return statement at the top level is not allowed"),
        }
    }
}

impl std::error::Error for ResolverError {}
