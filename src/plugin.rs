//! Plugin interface and metadata

use crate::{NodeMetadata, PluginError, NodeRegistryTrait, NodeCategory};

// Viewport rendering is now handled by the core using viewport data
// See viewport.rs for the new data-driven approach

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub compatible_version: String, // Nodle version compatibility
}

/// Menu structure for organizing nodes in the UI
#[derive(Debug, Clone)]
pub enum MenuStructure {
    Category {
        name: String,
        items: Vec<MenuStructure>,
    },
    Node {
        name: String,
        node_type: String,
        metadata: NodeMetadata,
    },
}


/// Main plugin trait that external libraries must implement
pub trait NodePlugin: Send + Sync {
    /// Plugin metadata
    fn plugin_info(&self) -> PluginInfo;
    
    /// Register all nodes provided by this plugin
    fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait);
    
    /// Get the menu structure for this plugin's nodes
    fn get_menu_structure(&self) -> Vec<MenuStructure> {
        Vec::new() // Default: no custom menu structure
    }
    
    /// Called when plugin is loaded (optional)
    fn on_load(&self) -> Result<(), PluginError> {
        Ok(())
    }
    
    /// Called when plugin is unloaded (optional)
    fn on_unload(&self) -> Result<(), PluginError> {
        Ok(())
    }
}

/// Concrete wrapper for safe FFI transfer
/// This avoids the undefined behavior of passing trait objects through extern "C"
#[repr(C)]
pub struct PluginHandle {
    plugin: *mut dyn NodePlugin,
}

impl PluginHandle {
    /// Create a new handle from a boxed plugin
    pub fn new(plugin: Box<dyn NodePlugin>) -> Self {
        Self {
            plugin: Box::into_raw(plugin),
        }
    }
    
    /// Convert back to a boxed plugin (takes ownership)
    pub unsafe fn into_plugin(self) -> Box<dyn NodePlugin> {
        Box::from_raw(self.plugin)
    }
    
    /// Get a reference to the plugin
    pub unsafe fn as_plugin(&self) -> &dyn NodePlugin {
        &*self.plugin
    }
    
    /// Get a mutable reference to the plugin
    pub unsafe fn as_plugin_mut(&mut self) -> &mut dyn NodePlugin {
        &mut *self.plugin
    }
}

/// Concrete wrapper for safe plugin node transfer
/// This avoids passing trait objects directly between plugin and core
#[repr(C)]
pub struct PluginNodeHandle {
    node: *mut dyn PluginNode,
}

impl PluginNodeHandle {
    /// Create a new handle from a boxed plugin node
    pub fn new(node: Box<dyn PluginNode>) -> Self {
        Self {
            node: Box::into_raw(node),
        }
    }
    
    /// Convert back to a boxed plugin node (takes ownership)
    pub unsafe fn into_node(self) -> Box<dyn PluginNode> {
        Box::from_raw(self.node)
    }
    
    /// Get a reference to the plugin node
    pub unsafe fn as_node(&self) -> &dyn PluginNode {
        &*self.node
    }
    
    /// Get a mutable reference to the plugin node
    pub unsafe fn as_node_mut(&mut self) -> &mut dyn PluginNode {
        &mut *self.node
    }
}

/// Node factory trait for creating nodes
pub trait NodeFactory: Send + Sync {
    /// Get metadata for this node type
    fn metadata(&self) -> NodeMetadata;
    
    /// Create a new node instance at the given position
    fn create_node(&self, position: egui::Pos2) -> PluginNodeHandle;
}

/// Simplified node interface for plugins
pub trait PluginNode: Send + Sync {
    /// Get the node's unique identifier
    fn id(&self) -> String;
    
    /// Get the node's position
    fn position(&self) -> egui::Pos2;
    
    /// Set the node's position
    fn set_position(&mut self, position: egui::Pos2);
    
    /// Get the parameter UI description
    fn get_parameter_ui(&self) -> ParameterUI;
    
    /// Handle UI actions
    fn handle_ui_action(&mut self, action: UIAction) -> Vec<ParameterChange>;
    
    /// Get a parameter value
    fn get_parameter(&self, name: &str) -> Option<NodeData>;
    
    /// Set a parameter value
    fn set_parameter(&mut self, name: &str, value: NodeData);
    
    /// Process the node (execute its functionality)
    fn process(&mut self, inputs: &std::collections::HashMap<String, NodeData>) -> std::collections::HashMap<String, NodeData>;
    
    /// Get viewport data for rendering (for viewport-type nodes)
    fn get_viewport_data(&self) -> Option<crate::viewport::ViewportData> {
        // Default implementation for non-viewport nodes
        None
    }
    
    /// Handle viewport camera manipulation (for viewport-type nodes)
    fn handle_viewport_camera(&mut self, manipulation: crate::viewport::CameraManipulation) {
        // Default implementation for non-viewport nodes
        // Does nothing
    }
    
    /// Handle viewport settings changes (for viewport-type nodes)
    fn handle_viewport_settings(&mut self, settings: crate::viewport::ViewportSettings) {
        // Default implementation for non-viewport nodes
        // Does nothing
    }
    
    /// Check if this node supports viewport rendering
    fn supports_viewport(&self) -> bool {
        false
    }
}

/// Parameter change notification
#[derive(Debug, Clone)]
pub struct ParameterChange {
    pub parameter: String,
    pub value: NodeData,
}

/// Normal Rust types for plugin UI
#[derive(Debug, Clone)]
pub enum UIElement {
    Heading(String),
    Label(String),
    Separator,
    TextEdit {
        label: String,
        value: String,
        parameter_name: String,
    },
    Checkbox {
        label: String,
        value: bool,
        parameter_name: String,
    },
    Button {
        label: String,
        action: String,
    },
    Slider {
        label: String,
        value: f32,
        min: f32,
        max: f32,
        parameter_name: String,
    },
    Vec3Edit {
        label: String,
        value: [f32; 3],
        parameter_name: String,
    },
    ColorEdit {
        label: String,
        value: [f32; 3],
        parameter_name: String,
    },
    Horizontal(Vec<UIElement>),
    Vertical(Vec<UIElement>),
}

#[derive(Debug, Clone)]
pub struct ParameterUI {
    pub elements: Vec<UIElement>,
}

#[derive(Debug, Clone)]
pub enum UIAction {
    ParameterChanged {
        parameter: String,
        value: NodeData,
    },
    ButtonClicked {
        action: String,
    },
}

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

