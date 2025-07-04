//! Plugin interface and metadata

use crate::{NodeMetadata, PluginError, NodeRegistryTrait};

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub compatible_version: String, // Nodle version compatibility
}

/// Main plugin trait that external libraries must implement
pub trait NodePlugin: Send + Sync {
    /// Plugin metadata
    fn plugin_info(&self) -> PluginInfo;
    
    /// Register all nodes provided by this plugin
    fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait);
    
    /// Called when plugin is loaded (optional)
    fn on_load(&self) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// Called when plugin is unloaded (optional)
    fn on_unload(&self) -> Result<(), PluginError> {
        Ok(())
    }
}

/// Node factory trait for creating nodes
pub trait NodeFactory: Send + Sync {
    /// Get metadata for this node type
    fn metadata(&self) -> NodeMetadata;
    
    /// Create a new node instance at the given position
    fn create_node(&self, position: egui::Pos2) -> Box<dyn PluginNode>;
}

/// Simplified node interface for plugins
pub trait PluginNode: Send + Sync {
    /// Get the node's unique identifier
    fn id(&self) -> String;
    
    /// Get the node's position
    fn position(&self) -> egui::Pos2;
    
    /// Set the node's position
    fn set_position(&mut self, position: egui::Pos2);
    
    /// Render the node's parameter interface
    fn render_parameters(&mut self, ui: &mut egui::Ui) -> Vec<ParameterChange>;
    
    /// Get a parameter value
    fn get_parameter(&self, name: &str) -> Option<NodeData>;
    
    /// Set a parameter value
    fn set_parameter(&mut self, name: &str, value: NodeData);
    
    /// Process the node (execute its functionality)
    fn process(&mut self, inputs: &std::collections::HashMap<String, NodeData>) -> std::collections::HashMap<String, NodeData>;
}

/// Parameter change notification
#[derive(Debug, Clone)]
pub struct ParameterChange {
    pub parameter: String,
    pub value: NodeData,
}

/// Node data types for parameter values
#[derive(Debug, Clone)]
pub enum NodeData {
    Float(f32),
    Vector3([f32; 3]),
    Color([f32; 3]),
    String(String),
    Boolean(bool),
}

impl NodeData {
    /// Try to extract as float
    pub fn as_float(&self) -> Option<f32> {
        match self {
            NodeData::Float(f) => Some(*f),
            _ => None,
        }
    }
    
    /// Try to extract as vector3
    pub fn as_vector3(&self) -> Option<[f32; 3]> {
        match self {
            NodeData::Vector3(v) => Some(*v),
            _ => None,
        }
    }
    
    /// Try to extract as string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            NodeData::String(s) => Some(s),
            _ => None,
        }
    }
    
    /// Try to extract as boolean
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            NodeData::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}