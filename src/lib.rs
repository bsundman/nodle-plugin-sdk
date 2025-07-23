//! Nodle Plugin SDK
//! 
//! This crate provides the complete interfaces and types needed to create full-featured plugins
//! for the Nodle node editor, including advanced caching, USD integration, and lifecycle hooks.
//! 
//! # Quick Start - Basic Plugin
//! 
//! ```rust
//! use nodle_plugin_sdk::*;
//! use std::collections::HashMap;
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
//! 
//! // Define your node
//! pub struct MyNode {
//!     multiplier: f32,
//! }
//! 
//! impl PluginNode for MyNode {
//!     fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
//!         let mut outputs = HashMap::new();
//!         
//!         if let Some(NodeData::Float(value)) = inputs.get("input") {
//!             outputs.insert("output".to_string(), NodeData::Float(value * self.multiplier));
//!         }
//!         
//!         outputs
//!     }
//!     
//!     // ... other required methods
//! }
//! ```
//! 
//! # Advanced Features
//! 
//! ## Multi-Stage Caching (like USD File Reader)
//! 
//! ```rust
//! use nodle_plugin_sdk::*;
//! use nodle_plugin_sdk::cache::strategies::MultiStageCache;
//! 
//! pub struct USDProcessorNode {
//!     cache: MultiStageCache,
//!     file_path: String,
//! }
//! 
//! impl PluginNode for USDProcessorNode {
//!     fn process_with_cache(
//!         &mut self, 
//!         inputs: &HashMap<String, NodeData>,
//!         cache: &mut dyn PluginCache,
//!         node_id: u32
//!     ) -> HashMap<String, NodeData> {
//!         // Stage 1: Load USD file (cached by file path)
//!         let usd_data = if let Some(cached) = self.cache.get_stage_cached(cache, node_id, "load", 0) {
//!             cached.clone()
//!         } else {
//!             let loaded = self.load_usd_file(&self.file_path).unwrap();
//!             self.cache.store_stage_result(cache, node_id, "load", 0, loaded.clone()).unwrap();
//!             loaded
//!         };
//!         
//!         // Stage 2: Process USD data (cached by parameters)
//!         let processed = if let Some(cached) = self.cache.get_stage_cached(cache, node_id, "process", 0) {
//!             cached.clone()
//!         } else {
//!             let processed = self.process_usd_data(&usd_data).unwrap();
//!             self.cache.store_stage_result(cache, node_id, "process", 0, processed.clone()).unwrap();
//!             processed
//!         };
//!         
//!         let mut outputs = HashMap::new();
//!         outputs.insert("output".to_string(), processed);
//!         outputs
//!     }
//! }
//! ```
//! 
//! ## Execution Hooks for Lifecycle Management
//! 
//! ```rust
//! use nodle_plugin_sdk::*;
//! 
//! #[derive(Clone)]
//! pub struct MyNodeHooks;
//! 
//! impl NodeExecutionHooks for MyNodeHooks {
//!     fn before_execution(&mut self, _handle: &PluginHandle, node_id: u32, _inputs: &HashMap<String, NodeData>) -> Result<(), String> {
//!         println!("Preparing node {} for execution", node_id);
//!         // Clear temporary caches, validate inputs, etc.
//!         Ok(())
//!     }
//!     
//!     fn on_parameter_changed(&mut self, _handle: &PluginHandle, node_id: u32, param: &str, _old: &NodeData, _new: &NodeData) -> Result<(), String> {
//!         println!("Parameter '{}' changed on node {}", param, node_id);
//!         // Invalidate specific caches based on which parameter changed
//!         Ok(())
//!     }
//!     
//!     fn clone_box(&self) -> Box<dyn NodeExecutionHooks> {
//!         Box::new(self.clone())
//!     }
//! }
//! 
//! impl PluginNode for MyNode {
//!     fn get_execution_hooks(&self) -> Option<Box<dyn NodeExecutionHooks>> {
//!         Some(Box::new(MyNodeHooks))
//!     }
//! }
//! ```

pub mod data_types;
pub mod metadata;
pub mod plugin;
pub mod registry;
pub mod errors;
pub mod viewport;
pub mod hooks;
pub mod cache;
pub mod ui;

// Re-export commonly used types
pub use data_types::*;
pub use metadata::*;
pub use plugin::*;
pub use registry::*;
pub use errors::*;
pub use hooks::*;
pub use cache::*;

// Specific re-exports from ui to avoid conflicts
pub use ui::{PanelType, InterfaceParameter, UIElement, ParameterChange, UIAction, ParameterUI};

// Specific re-exports from viewport to avoid conflicts  
pub use viewport::{CameraData, CameraManipulation, ViewportData, ViewportSettings, MeshData, ShadingMode};

// Data types are the authoritative source for SceneData, MaterialData, LightData, LightType

// Re-export egui for convenience
pub use egui::{Color32, Pos2, Vec2, Ui};