//! Plugin registry interface

use crate::{NodeFactory, PluginError};

/// Trait for registering nodes from plugins
pub trait NodeRegistryTrait {
    /// Register a node factory from a plugin
    fn register_node_factory(&mut self, factory: Box<dyn NodeFactory>) -> Result<(), PluginError>;
    
    /// Get list of registered node types
    fn get_node_types(&self) -> Vec<String>;
    
    /// Check if a node type is registered
    fn has_node_type(&self, node_type: &str) -> bool;
}