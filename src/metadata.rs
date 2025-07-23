//! Node metadata and categorization system

use crate::{DataType, PanelType};
use egui::{Color32, Vec2};
use serde::{Deserialize, Serialize};

/// Hierarchical category system for organizing nodes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeCategory {
    path: Vec<String>,
}

impl NodeCategory {
    /// Create a new category from path components
    pub fn new(path: &[&str]) -> Self {
        Self {
            path: path.iter().map(|s| s.to_string()).collect(),
        }
    }
    
    /// Get the full path as a slice
    pub fn path(&self) -> &[String] {
        &self.path
    }
    
    /// Get the category name (last component)
    pub fn name(&self) -> &str {
        self.path.last().map(|s| s.as_str()).unwrap_or("")
    }
    
    /// Get the parent category
    pub fn parent(&self) -> Option<NodeCategory> {
        if self.path.len() > 1 {
            Some(NodeCategory {
                path: self.path[..self.path.len() - 1].to_vec(),
            })
        } else {
            None
        }
    }
    
    /// Check if this category is a child of another
    pub fn is_child_of(&self, other: &NodeCategory) -> bool {
        self.path.len() > other.path.len() && 
        self.path[..other.path.len()] == other.path
    }
    
    /// Get display string for UI
    pub fn display_string(&self) -> String {
        self.path.join(" > ")
    }
}

// Standard categories
impl NodeCategory {
    /// Get standard math category
    pub fn math() -> Self { Self::new(&["Math"]) }
    /// Get standard logic category  
    pub fn logic() -> Self { Self::new(&["Logic"]) }
    /// Get standard data category
    pub fn data() -> Self { Self::new(&["Data"]) }
    /// Get standard output category
    pub fn output() -> Self { Self::new(&["Output"]) }
    /// Get utility category
    pub fn utility() -> Self { Self::new(&["Utility"]) }
    /// Get MaterialX shading category
    pub fn materialx_shading() -> Self { Self::new(&["MaterialX", "Shading"]) }
    /// Get MaterialX texture category
    pub fn materialx_texture() -> Self { Self::new(&["MaterialX", "Texture"]) }
    /// Get 3D transform category
    pub fn transform_3d() -> Self { Self::new(&["3D", "Transform"]) }
    /// Get 3D geometry category
    pub fn geometry_3d() -> Self { Self::new(&["3D", "Geometry"]) }
    /// Get 3D lighting category
    pub fn lighting_3d() -> Self { Self::new(&["3D", "Lighting"]) }
    /// Get Cycles rendering category
    pub fn cycles_rendering() -> Self { Self::new(&["3D", "Cycles", "Rendering"]) }
    /// Get Cycles material category
    pub fn cycles_material() -> Self { Self::new(&["3D", "Cycles", "Material"]) }
    /// Get Cycles lighting category
    pub fn cycles_lighting() -> Self { Self::new(&["3D", "Cycles", "Lighting"]) }
}

/// Port definition for node creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDefinition {
    pub name: String,
    pub data_type: DataType,
    pub optional: bool,
    pub description: Option<String>,
}

impl PortDefinition {
    /// Create a required port
    pub fn required(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            optional: false,
            description: None,
        }
    }
    
    /// Create an optional port
    pub fn optional(name: &str, data_type: DataType) -> Self {
        Self {
            name: name.to_string(),
            data_type,
            optional: true,
            description: None,
        }
    }
    
    /// Add description to port
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

/// Panel positioning preferences
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PanelPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Custom(Vec2), // Custom offset from top-left
}

/// Stacking behavior for panels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StackingMode {
    Floating,      // Individual windows
    VerticalStack, // Stacked vertically (parameter style)
    TabbedStack,   // Stacked with tabs (viewport style)
    Docked,        // Docked to window edges
}

/// Node execution behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionMode {
    Realtime,     // Executes continuously
    OnDemand,     // Executes when inputs change
    Manual,       // Executes only when triggered
    Background,   // Executes in background thread
}

/// Processing cost hint for scheduling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessingCost {
    Minimal,      // < 1ms
    Low,          // 1-10ms
    Medium,       // 10-100ms
    High,         // 100ms-1s
    VeryHigh,     // > 1s
}


/// Rich metadata for nodes - the single source of truth for all node behavior
#[derive(Debug, Clone)]
pub struct NodeMetadata {
    // Core identity
    pub node_type: String,
    pub display_name: String,
    pub description: String,
    pub version: String,
    
    // Visual appearance
    pub color: Color32,
    pub icon: String,
    pub size_hint: Vec2,
    
    // Organization & categorization
    pub category: NodeCategory,
    pub workspace_compatibility: Vec<String>,
    pub tags: Vec<String>,
    
    // Interface behavior
    pub panel_type: PanelType,
    pub default_panel_position: PanelPosition,
    pub default_stacking_mode: StackingMode,
    pub resizable: bool,
    
    // Connectivity
    pub inputs: Vec<PortDefinition>,
    pub outputs: Vec<PortDefinition>,
    pub allow_multiple_connections: bool,
    
    // Execution behavior
    pub execution_mode: ExecutionMode,
    pub processing_cost: ProcessingCost,
    pub requires_gpu: bool,
    
    // Advanced properties
    pub is_workspace_node: bool,
    pub supports_preview: bool,
}

impl NodeMetadata {
    /// Create node metadata with sensible defaults
    pub fn new(
        node_type: &str,
        display_name: &str,
        category: NodeCategory,
        description: &str,
    ) -> Self {
        Self {
            // Core identity
            node_type: node_type.to_string(),
            display_name: display_name.to_string(),
            description: description.to_string(),
            version: "1.0.0".to_string(),
            
            // Visual appearance
            color: Color32::from_rgb(100, 100, 200),
            icon: "●".to_string(),
            size_hint: Vec2::new(120.0, 80.0),
            
            // Organization & categorization
            category,
            workspace_compatibility: vec!["General".to_string()],
            tags: Vec::new(),
            
            // Interface behavior
            panel_type: PanelType::Parameter,
            default_panel_position: PanelPosition::TopRight,
            default_stacking_mode: StackingMode::VerticalStack,
            resizable: true,
            
            // Connectivity
            inputs: Vec::new(),
            outputs: Vec::new(),
            allow_multiple_connections: false,
            
            // Execution behavior
            execution_mode: ExecutionMode::OnDemand,
            processing_cost: ProcessingCost::Low,
            requires_gpu: false,
            
            // Advanced properties
            is_workspace_node: false,
            supports_preview: false,
        }
    }
    
    /// Set workspace compatibility
    pub fn with_workspace_compatibility(mut self, workspaces: Vec<&str>) -> Self {
        self.workspace_compatibility = workspaces.iter().map(|s| s.to_string()).collect();
        self
    }
    
    /// Set panel type
    pub fn with_panel_type(mut self, panel_type: PanelType) -> Self {
        self.panel_type = panel_type;
        self
    }
    
    /// Set node color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }
    
    /// Set node icon
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = icon.to_string();
        self
    }
    
    /// Add input ports
    pub fn with_inputs(mut self, inputs: Vec<PortDefinition>) -> Self {
        self.inputs = inputs;
        self
    }
    
    /// Add output ports
    pub fn with_outputs(mut self, outputs: Vec<PortDefinition>) -> Self {
        self.outputs = outputs;
        self
    }
}