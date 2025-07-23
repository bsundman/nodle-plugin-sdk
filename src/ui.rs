//! User interface components and parameter system for plugins
//!
//! This module provides rich UI components that match the main application's
//! interface system, allowing plugins to create sophisticated parameter panels.

use crate::NodeData;
use egui::{Color32, DragValue, Ui};
use serde::{Deserialize, Serialize};

/// Types of interface panels that nodes can specify
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelType {
    /// Parameter panels for node settings (default, positioned top-right)
    Parameter,
    /// Viewer panels for displaying output/results
    Viewer,
    /// Editor panels for complex editing interfaces
    Editor,
    /// Inspector panels for debugging/analysis
    Inspector,
    /// Viewport panels for 3D scene visualization and rendering
    Viewport,
    /// Tree panels for hierarchical scene graph visualization
    Tree,
    /// Spreadsheet panels for tabular data display
    Spreadsheet,
    /// Combined parameter and viewport panels
    Combined,
}

/// Advanced interface parameters that can be controlled in panels
/// This matches the main application's InterfaceParameter system exactly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterfaceParameter {
    Float { value: f32, min: f32, max: f32, step: f32 },
    Integer { value: i32, min: i32, max: i32 },
    Vector3 { value: [f32; 3] },
    Color { value: [f32; 4] },
    String { value: String },
    Boolean { value: bool },
    Enum { value: usize, options: Vec<String> },
    FilePath { value: String, filter: String },
}

impl InterfaceParameter {
    /// Render the parameter in the UI and return if it changed
    pub fn render(&mut self, ui: &mut Ui, label: &str) -> bool {
        match self {
            InterfaceParameter::Float { value, min, max, step } => {
                ui.add(DragValue::new(value)
                    .speed(*step)
                    .range(*min..=*max)
                    .prefix(format!("{}: ", label)))
                    .changed()
            }
            InterfaceParameter::Integer { value, min, max } => {
                ui.add(DragValue::new(value)
                    .range(*min..=*max)
                    .prefix(format!("{}: ", label)))
                    .changed()
            }
            InterfaceParameter::Vector3 { value } => {
                ui.horizontal(|ui| {
                    ui.label(label);
                    let mut changed = false;
                    changed |= ui.add(DragValue::new(&mut value[0]).prefix("X:")).changed();
                    changed |= ui.add(DragValue::new(&mut value[1]).prefix("Y:")).changed();
                    changed |= ui.add(DragValue::new(&mut value[2]).prefix("Z:")).changed();
                    changed
                }).inner
            }
            InterfaceParameter::Color { value } => {
                ui.horizontal(|ui| {
                    ui.label(label);
                    let mut color = Color32::from_rgba_premultiplied(
                        (value[0] * 255.0) as u8,
                        (value[1] * 255.0) as u8,
                        (value[2] * 255.0) as u8,
                        (value[3] * 255.0) as u8,
                    );
                    let changed = ui.color_edit_button_srgba(&mut color).changed();
                    if changed {
                        value[0] = color.r() as f32 / 255.0;
                        value[1] = color.g() as f32 / 255.0;
                        value[2] = color.b() as f32 / 255.0;
                        value[3] = color.a() as f32 / 255.0;
                    }
                    changed
                }).inner
            }
            InterfaceParameter::String { value } => {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.text_edit_singleline(value).changed()
                }).inner
            }
            InterfaceParameter::Boolean { value } => {
                ui.checkbox(value, label).changed()
            }
            InterfaceParameter::Enum { value, options } => {
                ui.horizontal(|ui| {
                    ui.label(label);
                    let mut changed = false;
                    egui::ComboBox::from_id_salt(label)
                        .selected_text(&options[*value])
                        .show_ui(ui, |ui| {
                            for (i, option) in options.iter().enumerate() {
                                if ui.selectable_value(value, i, option).changed() {
                                    changed = true;
                                }
                            }
                        });
                    changed
                }).inner
            }
            InterfaceParameter::FilePath { value, filter: _ } => {
                ui.horizontal(|ui| {
                    ui.label(label);
                    let mut changed = ui.text_edit_singleline(value).changed();
                    if ui.button("Browse").clicked() {
                        // In a real implementation, this would open a file dialog
                        // For now, just indicate that browsing was requested
                        changed = true;
                    }
                    changed
                }).inner
            }
        }
    }
    
    /// Get the current value as NodeData
    pub fn to_node_data(&self) -> NodeData {
        match self {
            InterfaceParameter::Float { value, .. } => NodeData::Float(*value),
            InterfaceParameter::Integer { value, .. } => NodeData::Integer(*value),
            InterfaceParameter::Vector3 { value } => NodeData::Vector3(*value),
            InterfaceParameter::Color { value } => NodeData::Color(*value),
            InterfaceParameter::String { value } => NodeData::String(value.clone()),
            InterfaceParameter::Boolean { value } => NodeData::Boolean(*value),
            InterfaceParameter::Enum { value, options } => NodeData::String(options[*value].clone()),
            InterfaceParameter::FilePath { value, .. } => NodeData::String(value.clone()),
        }
    }
    
    /// Update the parameter from NodeData
    pub fn from_node_data(&mut self, data: &NodeData) -> bool {
        match (self, data) {
            (InterfaceParameter::Float { value, .. }, NodeData::Float(new_value)) => {
                if *value != *new_value {
                    *value = *new_value;
                    true
                } else {
                    false
                }
            }
            (InterfaceParameter::Integer { value, .. }, NodeData::Integer(new_value)) => {
                if *value != *new_value {
                    *value = *new_value;
                    true
                } else {
                    false
                }
            }
            (InterfaceParameter::Vector3 { value }, NodeData::Vector3(new_value)) => {
                if *value != *new_value {
                    *value = *new_value;
                    true
                } else {
                    false
                }
            }
            (InterfaceParameter::Color { value }, NodeData::Color(new_value)) => {
                if *value != *new_value {
                    *value = *new_value;
                    true
                } else {
                    false
                }
            }
            (InterfaceParameter::String { value }, NodeData::String(new_value)) => {
                if value != new_value {
                    *value = new_value.clone();
                    true
                } else {
                    false
                }
            }
            (InterfaceParameter::Boolean { value }, NodeData::Boolean(new_value)) => {
                if *value != *new_value {
                    *value = *new_value;
                    true
                } else {
                    false
                }
            }
            (InterfaceParameter::Enum { value, options }, NodeData::String(new_value)) => {
                if let Some(index) = options.iter().position(|opt| opt == new_value) {
                    if *value != index {
                        *value = index;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            (InterfaceParameter::FilePath { value, .. }, NodeData::String(new_value)) => {
                if value != new_value {
                    *value = new_value.clone();
                    true
                } else {
                    false
                }
            }
            _ => false, // Type mismatch
        }
    }
}

/// Rich UI elements for plugin interfaces
/// This extends the basic UIElement system with more sophisticated components
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
    Slider {
        label: String,
        value: f32,
        min: f32,
        max: f32,
        parameter_name: String,
    },
    ColorPicker {
        label: String,
        value: [f32; 4],
        parameter_name: String,
    },
    ComboBox {
        label: String,
        selected: usize,
        options: Vec<String>,
        parameter_name: String,
    },
    Vector3Input {
        label: String,
        value: [f32; 3],
        parameter_name: String,
    },
    FilePicker {
        label: String,
        value: String,
        filter: String,
        parameter_name: String,
    },
    Button {
        label: String,
        action: String,
    },
    Group {
        label: String,
        children: Vec<UIElement>,
    },
    Collapsible {
        label: String,
        open: bool,
        children: Vec<UIElement>,
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

impl UIElement {
    /// Render this UI element and return any changes
    pub fn render(&mut self, ui: &mut Ui) -> Vec<ParameterChange> {
        let mut changes = Vec::new();
        
        match self {
            UIElement::Heading(text) => {
                ui.heading(text.as_str());
            }
            UIElement::Label(text) => {
                ui.label(text.as_str());
            }
            UIElement::Separator => {
                ui.separator();
            }
            UIElement::TextEdit { label, value, parameter_name } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    if ui.text_edit_singleline(value).changed() {
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::String(value.clone()),
                        });
                    }
                });
            }
            UIElement::Checkbox { label, value, parameter_name } => {
                if ui.checkbox(value, label.as_str()).changed() {
                    changes.push(ParameterChange {
                        parameter: parameter_name.clone(),
                        value: NodeData::Boolean(*value),
                    });
                }
            }
            UIElement::Slider { label, value, min, max, parameter_name } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    if ui.add(egui::Slider::new(value, *min..=*max)).changed() {
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::Float(*value),
                        });
                    }
                });
            }
            UIElement::ColorPicker { label, value, parameter_name } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    let mut color = Color32::from_rgba_premultiplied(
                        (value[0] * 255.0) as u8,
                        (value[1] * 255.0) as u8,
                        (value[2] * 255.0) as u8,
                        (value[3] * 255.0) as u8,
                    );
                    if ui.color_edit_button_srgba(&mut color).changed() {
                        value[0] = color.r() as f32 / 255.0;
                        value[1] = color.g() as f32 / 255.0;
                        value[2] = color.b() as f32 / 255.0;
                        value[3] = color.a() as f32 / 255.0;
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::Color(*value),
                        });
                    }
                });
            }
            UIElement::ComboBox { label, selected, options, parameter_name } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    let mut changed = false;
                    egui::ComboBox::from_id_salt(label)
                        .selected_text(&options[*selected])
                        .show_ui(ui, |ui| {
                            for (i, option) in options.iter().enumerate() {
                                if ui.selectable_value(selected, i, option).changed() {
                                    changed = true;
                                }
                            }
                        });
                    if changed {
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::String(options[*selected].clone()),
                        });
                    }
                });
            }
            UIElement::Vector3Input { label, value, parameter_name } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    let mut changed = false;
                    changed |= ui.add(DragValue::new(&mut value[0]).prefix("X:")).changed();
                    changed |= ui.add(DragValue::new(&mut value[1]).prefix("Y:")).changed();
                    changed |= ui.add(DragValue::new(&mut value[2]).prefix("Z:")).changed();
                    if changed {
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::Vector3(*value),
                        });
                    }
                });
            }
            UIElement::FilePicker { label, value, parameter_name, .. } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    let mut changed = ui.text_edit_singleline(value).changed();
                    if ui.button("Browse").clicked() {
                        // File dialog would be handled by the main application
                        changed = true;
                    }
                    if changed {
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::String(value.clone()),
                        });
                    }
                });
            }
            UIElement::Button { label, action } => {
                if ui.button(label.as_str()).clicked() {
                    changes.push(ParameterChange {
                        parameter: action.clone(),
                        value: NodeData::Boolean(true),
                    });
                }
            }
            UIElement::Group { label, children } => {
                ui.group(|ui| {
                    ui.label(label.as_str());
                    for child in children {
                        changes.extend(child.render(ui));
                    }
                });
            }
            UIElement::Collapsible { label, open, children } => {
                ui.collapsing(label.as_str(), |ui| {
                    *open = true;
                    for child in children {
                        changes.extend(child.render(ui));
                    }
                });
            }
            UIElement::Vec3Edit { label, value, parameter_name } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    let mut changed = false;
                    changed |= ui.add(DragValue::new(&mut value[0]).prefix("X:")).changed();
                    changed |= ui.add(DragValue::new(&mut value[1]).prefix("Y:")).changed();
                    changed |= ui.add(DragValue::new(&mut value[2]).prefix("Z:")).changed();
                    if changed {
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::Vector3(*value),
                        });
                    }
                }).inner;
            }
            UIElement::ColorEdit { label, value, parameter_name } => {
                ui.horizontal(|ui| {
                    ui.label(label.as_str());
                    let mut color = Color32::from_rgba_premultiplied(
                        (value[0] * 255.0) as u8,
                        (value[1] * 255.0) as u8,
                        (value[2] * 255.0) as u8,
                        255,
                    );
                    if ui.color_edit_button_srgba(&mut color).changed() {
                        value[0] = color.r() as f32 / 255.0;
                        value[1] = color.g() as f32 / 255.0;
                        value[2] = color.b() as f32 / 255.0;
                        changes.push(ParameterChange {
                            parameter: parameter_name.clone(),
                            value: NodeData::Color([value[0], value[1], value[2], 1.0]),
                        });
                    }
                }).inner;
            }
            UIElement::Horizontal(children) => {
                ui.horizontal(|ui| {
                    for child in children {
                        changes.extend(child.render(ui));
                    }
                }).inner;
            }
            UIElement::Vertical(children) => {
                ui.vertical(|ui| {
                    for child in children {
                        changes.extend(child.render(ui));
                    }
                }).inner;
            }
        }
        
        changes
    }
}

/// Parameter change notification
#[derive(Debug, Clone)]
pub struct ParameterChange {
    pub parameter: String,
    pub value: NodeData,
}

/// UI action types for plugin interaction
#[derive(Debug, Clone)]
pub enum UIAction {
    ButtonClicked { action: String },
    ParameterChanged { parameter: String, value: NodeData },
    FileSelected { parameter: String, path: String },
}

/// Parameter UI structure for plugins
#[derive(Debug, Clone)]
pub struct ParameterUI {
    pub elements: Vec<UIElement>,
}

impl ParameterUI {
    /// Create a new empty parameter UI
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
    
    /// Add an element to the UI
    pub fn add_element(&mut self, element: UIElement) {
        self.elements.push(element);
    }
    
    /// Add a heading
    pub fn add_heading(&mut self, text: impl Into<String>) {
        self.add_element(UIElement::Heading(text.into()));
    }
    
    /// Add a label
    pub fn add_label(&mut self, text: impl Into<String>) {
        self.add_element(UIElement::Label(text.into()));
    }
    
    /// Add a separator
    pub fn add_separator(&mut self) {
        self.add_element(UIElement::Separator);
    }
    
    /// Add a text input
    pub fn add_text_input(&mut self, label: impl Into<String>, value: impl Into<String>, parameter_name: impl Into<String>) {
        self.add_element(UIElement::TextEdit {
            label: label.into(),
            value: value.into(),
            parameter_name: parameter_name.into(),
        });
    }
    
    /// Add a checkbox
    pub fn add_checkbox(&mut self, label: impl Into<String>, value: bool, parameter_name: impl Into<String>) {
        self.add_element(UIElement::Checkbox {
            label: label.into(),
            value,
            parameter_name: parameter_name.into(),
        });
    }
    
    /// Add a slider
    pub fn add_slider(&mut self, label: impl Into<String>, value: f32, min: f32, max: f32, parameter_name: impl Into<String>) {
        self.add_element(UIElement::Slider {
            label: label.into(),
            value,
            min,
            max,
            parameter_name: parameter_name.into(),
        });
    }
    
    /// Add a color picker
    pub fn add_color_picker(&mut self, label: impl Into<String>, value: [f32; 4], parameter_name: impl Into<String>) {
        self.add_element(UIElement::ColorPicker {
            label: label.into(),
            value,
            parameter_name: parameter_name.into(),
        });
    }
    
    /// Add a combo box
    pub fn add_combo_box(&mut self, label: impl Into<String>, selected: usize, options: Vec<String>, parameter_name: impl Into<String>) {
        self.add_element(UIElement::ComboBox {
            label: label.into(),
            selected,
            options,
            parameter_name: parameter_name.into(),
        });
    }
    
    /// Add a vector3 input
    pub fn add_vector3_input(&mut self, label: impl Into<String>, value: [f32; 3], parameter_name: impl Into<String>) {
        self.add_element(UIElement::Vector3Input {
            label: label.into(),
            value,
            parameter_name: parameter_name.into(),
        });
    }
    
    /// Add a file picker
    pub fn add_file_picker(&mut self, label: impl Into<String>, value: impl Into<String>, filter: impl Into<String>, parameter_name: impl Into<String>) {
        self.add_element(UIElement::FilePicker {
            label: label.into(),
            value: value.into(),
            filter: filter.into(),
            parameter_name: parameter_name.into(),
        });
    }
    
    /// Add a button
    pub fn add_button(&mut self, label: impl Into<String>, action: impl Into<String>) {
        self.add_element(UIElement::Button {
            label: label.into(),
            action: action.into(),
        });
    }
    
    /// Render all elements and return any parameter changes
    pub fn render(&mut self, ui: &mut Ui) -> Vec<ParameterChange> {
        let mut changes = Vec::new();
        for element in &mut self.elements {
            changes.extend(element.render(ui));
        }
        changes
    }
}

impl Default for ParameterUI {
    fn default() -> Self {
        Self::new()
    }
}