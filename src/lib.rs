//! Nodle Plugin SDK
//! 
//! This crate provides the core interfaces and types needed to create plugins for the Nodle node editor.
//! 
//! # Quick Start
//! 
//! ```rust
//! use nodle_plugin_sdk::*;
//! 
//! // Define your plugin
//! pub struct MyPlugin;
//! 
//! impl NodePlugin for MyPlugin {
//!     fn plugin_info(&self) -> PluginInfo {
//!         PluginInfo {
//!             name: "My Plugin".to_string(),
//!             version: "0.1.0".to_string(),
//!             author: "Your Name".to_string(),
//!             description: "A sample plugin".to_string(),
//!             compatible_version: "0.1.0".to_string(),
//!         }
//!     }
//!     
//!     fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait) {
//!         registry.register_node_factory(Box::new(MyNodeFactory));
//!     }
//! }
//! ```

pub mod data_types;
pub mod metadata;
pub mod plugin;
pub mod registry;
pub mod errors;

// Re-export commonly used types
pub use data_types::*;
pub use metadata::*;
pub use plugin::*;
pub use registry::*;
pub use errors::*;

// Re-export egui for convenience
pub use egui::{Color32, Pos2, Vec2, Ui};