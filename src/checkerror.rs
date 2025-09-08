#[derive(Debug)]
pub enum CheckError {
    InvalidAddress(String),
    ConnectionFailed(String),
    RequestFailed(String),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::InvalidAddress(addr) => write!(f, "Invalid address: {}", addr),
            CheckError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            CheckError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
        }
    }
}

impl std::error::Error for CheckError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = CheckError::InvalidAddress("invalid".to_string());
        assert_eq!(error.to_string(), "Invalid address: invalid");

        let error = CheckError::ConnectionFailed("timeout".to_string());
        assert_eq!(error.to_string(), "Connection failed: timeout");

        let error = CheckError::RequestFailed("404".to_string());
        assert_eq!(error.to_string(), "Request failed: 404");
    }

    #[test]
    fn test_error_debug() {
        let error = CheckError::InvalidAddress("test".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidAddress"));
        assert!(debug_str.contains("test"));
    }
}
