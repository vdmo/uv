/// Error recovery policy: abort compilation after this many errors (`None` = unbounded).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorRecoveryPolicy {
    pub max_error_count: Option<usize>,
}

pub const SUGGESTED_MAX_ERROR_COUNT: usize = 100;

impl Default for ErrorRecoveryPolicy {
    fn default() -> Self {
        ErrorRecoveryPolicy { max_error_count: Some(SUGGESTED_MAX_ERROR_COUNT) }
    }
}
