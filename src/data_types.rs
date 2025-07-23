//! Data types that can flow through node ports
//! 
//! This module provides all the data types that flow between nodes in the Nodle system,
//! including complex 3D scene data, USD integration, and basic primitive types.

use egui::Color32;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core data types that flow between nodes
/// This matches the main application's NodeData enum exactly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeData {
    /// Complete 3D scene with geometry, materials, lights
    Scene(SceneData),
    /// Geometric data (meshes, primitives)
    Geometry(GeometryData),
    /// Material and shading data
    Material(MaterialData),
    /// USD stage reference
    Stage(StageData),
    /// Complete USD scene data with full geometry
    USDSceneData(USDSceneData),
    /// Lightweight USD metadata for scenegraph display (no geometry data)
    USDScenegraphMetadata(USDScenegraphMetadata),
    /// Lighting data
    Light(LightData),
    /// Image/texture data
    Image(ImageData),
    /// Generic value types
    Float(f32),
    Integer(i32),
    Vector3([f32; 3]),
    Color([f32; 4]),
    String(String),
    Boolean(bool),
    Any(String), // Generic reference/handle
    USDScene(String), // USD scene data as string for plugin interface
    None, // Empty/null value
}

impl NodeData {
    /// Try to extract as float
    pub fn as_float(&self) -> Option<f32> {
        match self {
            NodeData::Float(f) => Some(*f),
            _ => None,
        }
    }
    
    /// Try to extract as integer
    pub fn as_integer(&self) -> Option<i32> {
        match self {
            NodeData::Integer(i) => Some(*i),
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
    
    /// Try to extract as color
    pub fn as_color(&self) -> Option<[f32; 4]> {
        match self {
            NodeData::Color(c) => Some(*c),
            _ => None,
        }
    }
    
    /// Try to extract as string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            NodeData::String(s) => Some(s),
            NodeData::Any(s) => Some(s),
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
    
    /// Try to extract as USD scene data
    pub fn as_usd_scene(&self) -> Option<&USDSceneData> {
        match self {
            NodeData::USDSceneData(data) => Some(data),
            _ => None,
        }
    }
    
    /// Try to extract as USD scene string (for plugin interface)
    pub fn as_usd_scene_string(&self) -> Option<&str> {
        match self {
            NodeData::USDScene(s) => Some(s),
            _ => None,
        }
    }
    
    /// Try to extract as scene data
    pub fn as_scene(&self) -> Option<&SceneData> {
        match self {
            NodeData::Scene(data) => Some(data),
            _ => None,
        }
    }
    
    /// Try to extract as geometry data
    pub fn as_geometry(&self) -> Option<&GeometryData> {
        match self {
            NodeData::Geometry(data) => Some(data),
            _ => None,
        }
    }
    
    /// Try to extract as material data
    pub fn as_material(&self) -> Option<&MaterialData> {
        match self {
            NodeData::Material(data) => Some(data),
            _ => None,
        }
    }
    
    /// Check if this is a None/empty value
    pub fn is_none(&self) -> bool {
        matches!(self, NodeData::None)
    }
}

/// Port data types for connection validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataType {
    /// Floating point number
    Float,
    /// Integer number
    Integer,
    /// 3D vector (x, y, z)
    Vector3,
    /// RGBA color value
    Color,
    /// Text string
    String,
    /// Boolean value
    Boolean,
    /// Complete 3D scene
    Scene,
    /// Geometric data
    Geometry,
    /// Material data
    Material,
    /// USD stage reference
    Stage,
    /// USD scene data
    USDScene,
    /// USD scenegraph metadata
    USDScenegraph,
    /// Light data
    Light,
    /// Image/texture data
    Image,
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
            DataType::Integer => "Integer",
            DataType::Vector3 => "Vector3", 
            DataType::Color => "Color",
            DataType::String => "String",
            DataType::Boolean => "Boolean",
            DataType::Scene => "Scene",
            DataType::Geometry => "Geometry",
            DataType::Material => "Material",
            DataType::Stage => "USD Stage",
            DataType::USDScene => "USD Scene",
            DataType::USDScenegraph => "USD Scenegraph",
            DataType::Light => "Light",
            DataType::Image => "Image",
            DataType::Any => "Any",
        }
    }
    
    /// Get a color representing this data type
    pub fn color(&self) -> Color32 {
        match self {
            DataType::Float => Color32::from_rgb(100, 150, 255), // Blue
            DataType::Integer => Color32::from_rgb(80, 120, 200), // Dark blue
            DataType::Vector3 => Color32::from_rgb(255, 100, 100), // Red
            DataType::Color => Color32::from_rgb(255, 200, 100), // Orange
            DataType::String => Color32::from_rgb(100, 255, 100), // Green
            DataType::Boolean => Color32::from_rgb(255, 100, 255), // Magenta
            DataType::Scene => Color32::from_rgb(180, 130, 70), // Brown
            DataType::Geometry => Color32::from_rgb(200, 100, 150), // Pink
            DataType::Material => Color32::from_rgb(150, 200, 100), // Light green
            DataType::Stage => Color32::from_rgb(70, 130, 180), // Steel blue
            DataType::USDScene => Color32::from_rgb(90, 150, 200), // Light steel blue
            DataType::USDScenegraph => Color32::from_rgb(110, 170, 220), // Lighter steel blue
            DataType::Light => Color32::from_rgb(255, 255, 100), // Yellow
            DataType::Image => Color32::from_rgb(200, 150, 255), // Purple
            DataType::Any => Color32::from_rgb(150, 150, 150), // Gray
        }
    }
}

/// Scene hierarchy data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub geometry: Vec<GeometryData>,
    pub materials: Vec<MaterialData>,
    pub lights: Vec<LightData>,
    pub transforms: HashMap<String, [[f32; 4]; 4]>, // Transform matrices
}

impl Default for SceneData {
    fn default() -> Self {
        Self {
            geometry: Vec::new(),
            materials: Vec::new(),
            lights: Vec::new(),
            transforms: HashMap::new(),
        }
    }
}

/// Geometry data for 3D objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryData {
    pub id: String,
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub material_id: Option<String>,
}

/// Material and shading data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialData {
    pub id: String,
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub normal_map: Option<String>,
    pub diffuse_map: Option<String>,
}

/// USD stage reference data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageData {
    pub identifier: String,
    pub file_path: Option<String>,
    pub prims: Vec<String>,
}

/// Lighting data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightData {
    pub id: String,
    pub light_type: LightType,
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightType {
    Point,
    Directional { direction: [f32; 3] },
    Spot { direction: [f32; 3], cone_angle: f32 },
}

/// Image/texture data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub id: String,
    pub file_path: Option<String>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    RGB8,
    RGBA8,
    HDR,
}

// USD-specific data structures

/// Complete USD scene data with full geometry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USDSceneData {
    /// Original up-axis from USD file (Y, Z, or X)
    pub up_axis: String,
    /// All mesh geometry in the scene
    pub meshes: Vec<USDMeshGeometry>,
    /// All lights in the scene
    pub lights: Vec<USDLight>,
    /// All materials in the scene
    pub materials: Vec<USDMaterial>,
    /// Scene bounds (min, max)
    pub bounds: Option<([f32; 3], [f32; 3])>,
}

impl Default for USDSceneData {
    fn default() -> Self {
        Self {
            up_axis: "Y".to_string(),
            meshes: Vec::new(),
            lights: Vec::new(),
            materials: Vec::new(),
            bounds: None,
        }
    }
}

/// USD mesh geometry data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USDMeshGeometry {
    /// Prim path in USD stage
    pub prim_path: String,
    /// Display name for UI
    pub display_name: String,
    /// Mesh vertices in 3D space
    pub vertices: Vec<[f32; 3]>,
    /// Triangle indices
    pub indices: Vec<u32>,
    /// Vertex normals
    pub normals: Vec<[f32; 3]>,
    /// UV coordinates
    pub uvs: Vec<[f32; 2]>,
    /// Vertex colors (if present)
    pub vertex_colors: Vec<[f32; 3]>,
    /// Transform matrix
    pub transform: [[f32; 4]; 4],
    /// Material binding
    pub material_path: Option<String>,
    /// Custom attributes/primvars
    pub primvars: HashMap<String, USDPrimvar>,
}

/// USD primvar (primitive variable) data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USDPrimvar {
    pub name: String,
    pub interpolation: PrimvarInterpolation,
    pub values: PrimvarValues,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimvarInterpolation {
    Constant,
    Uniform,
    Varying,
    Vertex,
    FaceVarying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimvarValues {
    Float(Vec<f32>),
    Vec2(Vec<[f32; 2]>),
    Vec3(Vec<[f32; 3]>),
    Vec4(Vec<[f32; 4]>),
    Int(Vec<i32>),
    String(Vec<String>),
}

/// USD light data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USDLight {
    pub prim_path: String,
    pub display_name: String,
    pub light_type: USDLightType,
    pub transform: [[f32; 4]; 4],
    pub color: [f32; 3],
    pub intensity: f32,
    pub exposure: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum USDLightType {
    Distant,
    Sphere,
    Rect,
    Disk,
    Cylinder,
}

/// USD material data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USDMaterial {
    pub prim_path: String,
    pub display_name: String,
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub metallic: f32,
    pub roughness: f32,
    pub opacity: f32,
    pub emission_color: [f32; 3],
    pub normal_map: Option<String>,
    pub diffuse_map: Option<String>,
}

/// Lightweight USD metadata for scenegraph display (no heavy geometry data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USDScenegraphMetadata {
    /// Scene hierarchy information
    pub hierarchy: Vec<USDPrimInfo>,
    /// Total primitive counts
    pub total_meshes: usize,
    pub total_lights: usize,
    pub total_materials: usize,
    /// Scene bounds
    pub bounds: Option<([f32; 3], [f32; 3])>,
    /// Up axis
    pub up_axis: String,
}

/// USD primitive information for scenegraph display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USDPrimInfo {
    pub path: String,
    pub name: String,
    pub prim_type: String,
    pub children: Vec<USDPrimInfo>,
    pub has_geometry: bool,
    pub has_material: bool,
    pub vertex_count: Option<usize>,
    pub triangle_count: Option<usize>,
}