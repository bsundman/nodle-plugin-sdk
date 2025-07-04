//! Plugin system errors

use std::fmt;

/// Errors that can occur in the plugin system
#[derive(Debug)]
pub enum PluginError {
    /// Plugin loading failed
    LoadError(String),
    /// Plugin initialization failed
    InitError(String),
    /// Plugin registration failed
    RegistrationError(String),
    /// Version compatibility issue
    CompatibilityError(String),
    /// Generic plugin error
    Other(String),
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginError::LoadError(msg) => write!(f, "Plugin load error: {}", msg),
            PluginError::InitError(msg) => write!(f, "Plugin initialization error: {}", msg),
            PluginError::RegistrationError(msg) => write!(f, "Plugin registration error: {}", msg),
            PluginError::CompatibilityError(msg) => write!(f, "Plugin compatibility error: {}", msg),
            PluginError::Other(msg) => write!(f, "Plugin error: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}