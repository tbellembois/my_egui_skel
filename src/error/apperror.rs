use std::fmt;

#[allow(dead_code)]
pub enum AppError {
    TestError,
    InternalError(String),
}

// Implement std::fmt::Display for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::TestError => write!(f, "just a test error, nothing anormal"),
            AppError::InternalError(e) => write!(f, "error: {}", e),
        }
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
