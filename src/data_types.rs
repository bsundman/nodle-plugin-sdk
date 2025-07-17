//! Data types that can flow through node ports

use egui::Color32;
use serde::{Deserialize, Serialize};

/// Data types that can flow through ports
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    /// Floating point number
    Float,
    /// 3D vector (x, y, z)
    Vector3,
    /// RGB color value
    Color,
    /// Text string
    String,
    /// Boolean value
    Boolean,
    /// USD scene data
    USDScene,
    /// Any type (for generic ports)
    Any,
}

impl DataType {
    /// Check if this data type can connect to another
    pub fn can_connect_to(&self, other: &DataType) -> bool {
        self == other || *self == DataType::Any || *other == DataType::Any
    }
    
    /// Get a human-readable name for this data type
    pub fn name(&self) -> &'static str {
        match self {
            DataType::Float => "Float",
            DataType::Vector3 => "Vector3", 
            DataType::Color => "Color",
            DataType::String => "String",
            DataType::Boolean => "Boolean",
            DataType::USDScene => "USD Scene",
            DataType::Any => "Any",
        }
    }
    
    /// Get a color representing this data type
    pub fn color(&self) -> Color32 {
        match self {
            DataType::Float => Color32::from_rgb(100, 150, 255), // Blue
            DataType::Vector3 => Color32::from_rgb(255, 100, 100), // Red
            DataType::Color => Color32::from_rgb(255, 200, 100), // Orange
            DataType::String => Color32::from_rgb(100, 255, 100), // Green
            DataType::Boolean => Color32::from_rgb(255, 100, 255), // Magenta
            DataType::USDScene => Color32::from_rgb(70, 130, 180), // Steel blue
            DataType::Any => Color32::from_rgb(150, 150, 150), // Gray
        }
    }
}