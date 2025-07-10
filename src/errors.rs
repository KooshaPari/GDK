//! Error types and handling for the GDK system
//!
//! Provides comprehensive error categorization and context for debugging:
//! - Git operation failures with detailed context
//! - Validation errors with specific rule violations
//! - Convergence analysis errors with mathematical context
//! - Thread management errors with file-specific details
//! - Agent workflow errors with session context

use thiserror::Error;

/// Comprehensive error types for all GDK operations
#[derive(Error, Debug)]
pub enum GdkError {
    /// Git repository operation failed
    #[error("Git operation failed: {operation}")]
    GitError {
        operation: String,
        #[source]
        source: git2::Error,
    },

    /// File system operation error
    #[error("File system error for path '{path}': {message}")]
    FileSystemError {
        path: String,
        message: String,
        #[source]
        source: std::io::Error,
    },

    /// Validation rule violation
    #[error("Validation failed for {rule}: {context}")]
    ValidationError {
        rule: String,
        context: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Mathematical convergence analysis error
    #[error("Convergence analysis failed: {reason} (attempted {iterations} iterations)")]
    ConvergenceError {
        reason: String,
        iterations: u32,
        last_score: f64,
        threshold: f64,
    },

    /// Thread management operation failed
    #[error("Thread error for file '{file_path}': {operation}")]
    ThreadError {
        file_path: String,
        operation: String,
        thread_id: uuid::Uuid,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Agent workflow management error
    #[error("Agent workflow error for '{agent_id}': {operation}")]
    AgentError {
        agent_id: String,
        operation: String,
        session_id: Option<uuid::Uuid>,
        context: String,
    },

    /// Serialization/deserialization error
    #[error("Serialization error: {format} - {context}")]
    SerializationError {
        format: String,
        context: String,
        #[source]
        source: serde_json::Error,
    },

    /// Configuration or setup error
    #[error("Configuration error: {setting} - {message}")]
    ConfigurationError {
        setting: String,
        message: String,
        suggested_fix: Option<String>,
    },

    /// Visualization generation error
    #[error("Visualization error for format '{format}': {operation}")]
    VisualizationError {
        format: String,
        operation: String,
        node_count: usize,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl GdkError {
    /// Create a git error with operation context
    pub fn git_error(operation: impl Into<String>, source: git2::Error) -> Self {
        Self::GitError {
            operation: operation.into(),
            source,
        }
    }

    /// Create a file system error with path context
    pub fn file_system_error(
        path: impl Into<String>,
        message: impl Into<String>,
        source: std::io::Error,
    ) -> Self {
        Self::FileSystemError {
            path: path.into(),
            message: message.into(),
            source,
        }
    }

    /// Create a validation error with detailed context
    pub fn validation_error(
        rule: impl Into<String>,
        context: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self::ValidationError {
            rule: rule.into(),
            context: context.into(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                details.into(),
            )),
        }
    }

    /// Create a convergence error with analysis context
    pub fn convergence_error(
        reason: impl Into<String>,
        iterations: u32,
        last_score: f64,
        threshold: f64,
    ) -> Self {
        Self::ConvergenceError {
            reason: reason.into(),
            iterations,
            last_score,
            threshold,
        }
    }

    /// Create a thread error with file and operation context
    pub fn thread_error(
        file_path: impl Into<String>,
        operation: impl Into<String>,
        thread_id: uuid::Uuid,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self::ThreadError {
            file_path: file_path.into(),
            operation: operation.into(),
            thread_id,
            source,
        }
    }

    /// Create an agent error with workflow context
    pub fn agent_error(
        agent_id: impl Into<String>,
        operation: impl Into<String>,
        session_id: Option<uuid::Uuid>,
        context: impl Into<String>,
    ) -> Self {
        Self::AgentError {
            agent_id: agent_id.into(),
            operation: operation.into(),
            session_id,
            context: context.into(),
        }
    }

    /// Create a configuration error with suggested fix
    pub fn configuration_error(
        setting: impl Into<String>,
        message: impl Into<String>,
        suggested_fix: Option<String>,
    ) -> Self {
        Self::ConfigurationError {
            setting: setting.into(),
            message: message.into(),
            suggested_fix,
        }
    }

    /// Get the error category for metrics and logging
    pub fn category(&self) -> &'static str {
        match self {
            Self::GitError { .. } => "git",
            Self::FileSystemError { .. } => "filesystem",
            Self::ValidationError { .. } => "validation",
            Self::ConvergenceError { .. } => "convergence",
            Self::ThreadError { .. } => "thread",
            Self::AgentError { .. } => "agent",
            Self::SerializationError { .. } => "serialization",
            Self::ConfigurationError { .. } => "configuration",
            Self::VisualizationError { .. } => "visualization",
        }
    }

    /// Check if this error is recoverable through retry
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::GitError { source, .. } => {
                // Some git errors are recoverable (network, locks)
                matches!(source.code(), git2::ErrorCode::Locked)
            }
            Self::FileSystemError { source, .. } => {
                // IO errors like permission issues might be recoverable
                matches!(source.kind(), std::io::ErrorKind::PermissionDenied | std::io::ErrorKind::TimedOut)
            }
            Self::ValidationError { .. } => false, // Code issues need fixing
            Self::ConvergenceError { .. } => true, // Can retry convergence
            Self::ThreadError { .. } => true,      // Thread ops might succeed on retry
            Self::AgentError { .. } => true,       // Agent ops might succeed
            Self::SerializationError { .. } => false, // Data format issues
            Self::ConfigurationError { .. } => false, // Config needs fixing
            Self::VisualizationError { .. } => true,  // Visualization might succeed
        }
    }
}

/// Result type alias for GDK operations
pub type GdkResult<T> = Result<T, GdkError>;

/// Extension trait for adding GDK-specific context to Results
pub trait GdkResultExt<T> {
    /// Add git operation context
    fn with_git_context(self, operation: &str) -> GdkResult<T>;
    
    /// Add file path context
    fn with_file_context(self, path: &str, operation: &str) -> GdkResult<T>;
    
    /// Add agent context
    fn with_agent_context(self, agent_id: &str, operation: &str) -> GdkResult<T>;
}

impl<T> GdkResultExt<T> for Result<T, git2::Error> {
    fn with_git_context(self, operation: &str) -> GdkResult<T> {
        self.map_err(|e| GdkError::git_error(operation, e))
    }
    
    fn with_file_context(self, _path: &str, operation: &str) -> GdkResult<T> {
        self.map_err(|e| GdkError::git_error(operation, e))
    }
    
    fn with_agent_context(self, _agent_id: &str, operation: &str) -> GdkResult<T> {
        self.map_err(|e| GdkError::git_error(operation, e))
    }
}

impl<T> GdkResultExt<T> for Result<T, std::io::Error> {
    fn with_git_context(self, operation: &str) -> GdkResult<T> {
        self.map_err(|e| GdkError::file_system_error("unknown", operation, e))
    }
    
    fn with_file_context(self, path: &str, operation: &str) -> GdkResult<T> {
        self.map_err(|e| GdkError::file_system_error(path, operation, e))
    }
    
    fn with_agent_context(self, agent_id: &str, operation: &str) -> GdkResult<T> {
        self.map_err(|e| GdkError::file_system_error(agent_id, operation, e))
    }
}

/// Implement conversion from tokio::task::JoinError
impl From<tokio::task::JoinError> for GdkError {
    fn from(err: tokio::task::JoinError) -> Self {
        GdkError::ValidationError {
            rule: "task_join".to_string(),
            context: "Async task failed to join".to_string(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            )),
        }
    }
}

/// Implement conversion from anyhow::Error for compatibility
impl From<anyhow::Error> for GdkError {
    fn from(err: anyhow::Error) -> Self {
        GdkError::ValidationError {
            rule: "anyhow_conversion".to_string(),
            context: format!("Converted from anyhow: {}", err),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            )),
        }
    }
}

/// Implement conversion from git2::Error
impl From<git2::Error> for GdkError {
    fn from(err: git2::Error) -> Self {
        GdkError::GitError {
            operation: "git_operation".to_string(),
            source: err,
        }
    }
}

/// Implement conversion from std::io::Error
impl From<std::io::Error> for GdkError {
    fn from(err: std::io::Error) -> Self {
        GdkError::FileSystemError {
            path: "unknown".to_string(),
            message: "IO operation failed".to_string(),
            source: err,
        }
    }
}

/// Implement conversion from std::time::SystemTimeError
impl From<std::time::SystemTimeError> for GdkError {
    fn from(err: std::time::SystemTimeError) -> Self {
        GdkError::ValidationError {
            rule: "system_time".to_string(),
            context: "Failed to get system time".to_string(),
            source: Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            )),
        }
    }
}