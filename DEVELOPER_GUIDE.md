# Nodle Plugin SDK - Developer Guide

## Overview

The Nodle Plugin SDK provides a comprehensive, modern interface for creating high-performance plugins for the Nodle node editor. This guide covers the complete feature set, from basic plugins to advanced multi-stage caching and lifecycle management.

## Recent Major Updates (2024)

### Complete Feature Parity
- **Rich NodeData System**: Full compatibility with main application's data types including Scene, Geometry, Material, USD, Light, and Image data
- **Advanced Caching**: Multi-stage caching system with intelligent invalidation strategies
- **Execution Hooks**: Complete lifecycle management with before/after execution hooks
- **Modern UI System**: Rich parameter interfaces with all modern UI components
- **USD Integration**: Native support for USD scene data processing
- **Performance Optimization**: Direct access to unified cache system for maximum performance

## Architecture Overview

### Core Components

1. **Plugin System** (`plugin.rs`)
   - NodePlugin trait for main plugin interface
   - NodeFactory for creating node instances
   - PluginNode for individual node implementation
   - Complete lifecycle management

2. **Data System** (`data_types.rs`)
   - Complete NodeData enum matching main application
   - USD data structures (USDSceneData, USDMeshGeometry, etc.)
   - Scene hierarchy and material data
   - Type conversion utilities

3. **UI System** (`ui.rs`)
   - InterfaceParameter system for rich parameter controls
   - UIElement components for complex interfaces
   - ParameterUI builder for organized panels
   - All modern UI components (sliders, color pickers, file dialogs, etc.)

4. **Caching System** (`cache.rs`)
   - PluginCache trait for main application integration
   - Multi-stage caching strategies
   - Intelligent cache invalidation
   - Performance monitoring and statistics

5. **Hooks System** (`hooks.rs`)
   - NodeExecutionHooks for lifecycle management
   - Parameter change notifications
   - Connection event handling
   - Smart cache invalidation triggers

## Plugin Development Patterns

### 1. Basic Plugin

For simple data transformation plugins:

```rust
use nodle_plugin_sdk::*;
use std::collections::HashMap;

pub struct BasicMathPlugin;

impl NodePlugin for BasicMathPlugin {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Basic Math".to_string(),
            version: "1.0.0".to_string(),
            author: "Developer".to_string(),
            description: "Basic mathematical operations".to_string(),
            compatible_version: "0.1.0".to_string(),
        }
    }

    fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait) {
        registry.register_node_factory(Box::new(MultiplyNodeFactory)).unwrap();
    }
}

struct MultiplyNode {
    multiplier: f32,
}

impl PluginNode for MultiplyNode {
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        let mut outputs = HashMap::new();
        
        if let Some(NodeData::Float(value)) = inputs.get("input") {
            outputs.insert("output".to_string(), NodeData::Float(value * self.multiplier));
        }
        
        outputs
    }
    
    fn get_parameter_ui(&self) -> ParameterUI {
        let mut ui = ParameterUI::new();
        ui.add_slider("Multiplier", self.multiplier, 0.0, 10.0, "multiplier");
        ui
    }
}
```

### 2. Advanced Plugin with Caching

For complex plugins that benefit from multi-stage caching:

```rust
use nodle_plugin_sdk::*;
use nodle_plugin_sdk::cache::strategies::MultiStageCache;

pub struct USDProcessorPlugin;

struct USDProcessorNode {
    cache_strategy: MultiStageCache,
    file_path: String,
    processing_quality: f32,
}

impl PluginNode for USDProcessorNode {
    fn process_with_cache(
        &mut self, 
        inputs: &HashMap<String, NodeData>,
        cache: &mut dyn PluginCache,
        node_id: u32
    ) -> HashMap<String, NodeData> {
        // Stage 1: Load USD file (cached by file path)
        let usd_data = if let Some(cached) = self.cache_strategy.get_stage_cached(cache, node_id, "load", 0) {
            println!("🔥 USD load cache hit");
            cached.clone()
        } else {
            println!("💾 Loading USD file: {}", &self.file_path);
            let loaded = self.load_usd_file(&self.file_path).unwrap();
            self.cache_strategy.store_stage_result(cache, node_id, "load", 0, loaded.clone()).unwrap();
            loaded
        };

        // Stage 2: Process USD data (cached by processing parameters)
        let processed = if let Some(cached) = self.cache_strategy.get_stage_cached(cache, node_id, "process", 0) {
            println!("🔥 USD processing cache hit");
            cached.clone()
        } else {
            println!("🔧 Processing USD data with quality: {}", self.processing_quality);
            let processed = self.process_usd_data(&usd_data, self.processing_quality).unwrap();
            self.cache_strategy.store_stage_result(cache, node_id, "process", 0, processed.clone()).unwrap();
            processed
        };

        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), processed);
        outputs
    }

    fn get_execution_hooks(&self) -> Option<Box<dyn NodeExecutionHooks>> {
        Some(Box::new(USDProcessorHooks))
    }

    fn get_parameter_ui(&self) -> ParameterUI {
        let mut ui = ParameterUI::new();
        
        ui.add_heading("File Input");
        ui.add_file_picker("USD File", &self.file_path, "USD Files (*.usd)|*.usd", "file_path");
        
        ui.add_separator();
        ui.add_heading("Processing");
        ui.add_slider("Quality", self.processing_quality, 0.1, 2.0, "quality");
        ui.add_checkbox("High Quality", self.processing_quality > 1.0, "high_quality");
        
        ui
    }

    // ... other methods
}

#[derive(Clone)]
struct USDProcessorHooks;

impl NodeExecutionHooks for USDProcessorHooks {
    fn on_parameter_changed(
        &mut self, 
        _handle: &PluginHandle, 
        node_id: u32, 
        param: &str, 
        _old: &NodeData, 
        _new: &NodeData
    ) -> Result<(), String> {
        match param {
            "file_path" => {
                println!("📁 File path changed - invalidating all caches for node {}", node_id);
                // File change invalidates everything
            },
            "quality" | "high_quality" => {
                println!("🔧 Processing parameter changed - invalidating Stage 2 cache for node {}", node_id);
                // Processing parameters only invalidate Stage 2
            },
            _ => {}
        }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn NodeExecutionHooks> {
        Box::new(self.clone())
    }
}
```

### 3. Rich UI Plugin

For plugins with complex user interfaces:

```rust
struct MaterialEditorNode {
    base_color: [f32; 4],
    metallic: f32,
    roughness: f32,
    normal_map_path: String,
    material_type: usize,
}

impl PluginNode for MaterialEditorNode {
    fn get_parameter_ui(&self) -> ParameterUI {
        let mut ui = ParameterUI::new();
        
        // Material Type Selection
        ui.add_heading("Material Type");
        ui.add_combo_box(
            "Type", 
            self.material_type, 
            vec!["Metal".to_string(), "Plastic".to_string(), "Glass".to_string()], 
            "material_type"
        );
        
        ui.add_separator();
        
        // Color Properties
        ui.add_heading("Color Properties");
        ui.add_color_picker("Base Color", self.base_color, "base_color");
        
        ui.add_separator();
        
        // Physical Properties
        ui.add_heading("Physical Properties");
        ui.add_slider("Metallic", self.metallic, 0.0, 1.0, "metallic");
        ui.add_slider("Roughness", self.roughness, 0.0, 1.0, "roughness");
        
        ui.add_separator();
        
        // Texture Maps
        ui.add_heading("Texture Maps");
        ui.add_file_picker(
            "Normal Map", 
            &self.normal_map_path, 
            "Image Files (*.png,*.jpg,*.exr)|*.png;*.jpg;*.exr", 
            "normal_map"
        );
        
        // Advanced Section
        let mut advanced_section = vec![
            UIElement::Vector3Input {
                label: "Scale".to_string(),
                value: [1.0, 1.0, 1.0],
                parameter_name: "scale".to_string(),
            },
            UIElement::Checkbox {
                label: "Two-Sided".to_string(),
                value: false,
                parameter_name: "two_sided".to_string(),
            },
        ];
        
        ui.add_element(UIElement::Collapsible {
            label: "Advanced Options".to_string(),
            open: false,
            children: advanced_section,
        });
        
        ui
    }
    
    // ... other methods
}
```

## Data Types and USD Integration

### Working with USD Data

```rust
impl PluginNode for USDAnalyzerNode {
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        let mut outputs = HashMap::new();
        
        if let Some(NodeData::USDSceneData(usd_scene)) = inputs.get("usd_input") {
            // Access USD scene data
            println!("USD Scene up axis: {}", usd_scene.up_axis);
            println!("Mesh count: {}", usd_scene.meshes.len());
            println!("Light count: {}", usd_scene.lights.len());
            
            // Process mesh data
            let total_vertices: usize = usd_scene.meshes.iter()
                .map(|mesh| mesh.vertices.len())
                .sum();
                
            // Create summary output
            let summary = format!(
                "USD Scene Analysis:\n- Meshes: {}\n- Lights: {}\n- Total Vertices: {}",
                usd_scene.meshes.len(),
                usd_scene.lights.len(),
                total_vertices
            );
            
            outputs.insert("summary".to_string(), NodeData::String(summary));
            
            // Pass through the USD data
            outputs.insert("usd_output".to_string(), NodeData::USDSceneData(usd_scene.clone()));
        }
        
        outputs
    }
}
```

### Creating Scene Data

```rust
impl PluginNode for SceneBuilderNode {
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        let mut scene = SceneData::default();
        
        // Add geometry
        if let Some(NodeData::Geometry(geometry)) = inputs.get("geometry") {
            scene.geometry.push(geometry.clone());
        }
        
        // Add materials
        if let Some(NodeData::Material(material)) = inputs.get("material") {
            scene.materials.push(material.clone());
        }
        
        // Add lighting
        if let Some(NodeData::Light(light)) = inputs.get("light") {
            scene.lights.push(light.clone());
        }
        
        // Add transforms
        scene.transforms.insert(
            "main_object".to_string(),
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        );
        
        let mut outputs = HashMap::new();
        outputs.insert("scene".to_string(), NodeData::Scene(scene));
        outputs
    }
}
```

## Cache Management Strategies

### Simple Caching

```rust
use nodle_plugin_sdk::cache::strategies::SimpleCache;

struct SimpleProcessorNode {
    cache: SimpleCache,
}

impl PluginNode for SimpleProcessorNode {
    fn process_with_cache(
        &mut self, 
        inputs: &HashMap<String, NodeData>,
        cache: &mut dyn PluginCache,
        node_id: u32
    ) -> HashMap<String, NodeData> {
        // Try cache first
        if let Some(cached_result) = self.cache.get_cached(cache, node_id, 0) {
            println!("🔥 Cache hit!");
            let mut outputs = HashMap::new();
            outputs.insert("output".to_string(), cached_result.clone());
            return outputs;
        }
        
        // Process and cache
        let result = self.expensive_processing(inputs);
        let _ = self.cache.store_result(cache, node_id, 0, result.clone());
        
        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), result);
        outputs
    }
}
```

### Multi-Stage Caching

```rust
use nodle_plugin_sdk::cache::strategies::MultiStageCache;

struct ComplexProcessorNode {
    cache: MultiStageCache,
}

impl PluginNode for ComplexProcessorNode {
    fn process_with_cache(
        &mut self, 
        inputs: &HashMap<String, NodeData>,
        cache: &mut dyn PluginCache,
        node_id: u32
    ) -> HashMap<String, NodeData> {
        // Stage 1: Data preparation
        let prepared_data = if let Some(cached) = self.cache.get_stage_cached(cache, node_id, "prepare", 0) {
            cached.clone()
        } else {
            let prepared = self.prepare_data(inputs);
            let _ = self.cache.store_stage_result(cache, node_id, "prepare", 0, prepared.clone());
            prepared
        };
        
        // Stage 2: Processing
        let processed_data = if let Some(cached) = self.cache.get_stage_cached(cache, node_id, "process", 0) {
            cached.clone()
        } else {
            let processed = self.process_data(&prepared_data);
            let _ = self.cache.store_stage_result(cache, node_id, "process", 0, processed.clone());
            processed
        };
        
        // Stage 3: Finalization
        let final_data = if let Some(cached) = self.cache.get_stage_cached(cache, node_id, "finalize", 0) {
            cached.clone()
        } else {
            let finalized = self.finalize_data(&processed_data);
            let _ = self.cache.store_stage_result(cache, node_id, "finalize", 0, finalized.clone());
            finalized
        };
        
        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), final_data);
        outputs
    }
}
```

## Performance Best Practices

### 1. Use Appropriate Caching Strategy
- **Simple caching** for single-step operations
- **Multi-stage caching** for file loading + processing patterns
- **Cache invalidation** only when necessary (use hooks to track parameter changes)

### 2. Efficient Data Structures
- Use references where possible to avoid cloning large data structures
- Consider `take()` for moving data out of cache when appropriate
- Use `contains()` to check cache existence before expensive `get()` calls

### 3. Smart Hook Implementation
- Only invalidate affected cache stages in `on_parameter_changed`
- Use parameter names to determine which caches to invalidate
- Implement `clone_box()` efficiently for hook lifecycle management

### 4. UI Performance
- Use `ParameterUI` builder pattern for organized interfaces
- Group related parameters in collapsible sections
- Use appropriate UI components for data types (sliders for continuous values, combo boxes for discrete choices)

## Build and Deployment

### Project Structure
```
my-plugin/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Plugin exports and FFI functions
│   ├── plugin.rs       # Main plugin implementation
│   ├── nodes/          # Individual node implementations
│   │   ├── mod.rs
│   │   ├── processor.rs
│   │   └── analyzer.rs
│   └── utils.rs        # Helper functions
├── assets/             # Optional: shader files, resources
└── README.md
```

### Cargo.toml Configuration
```toml
[package]
name = "my-nodle-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
nodle-plugin-sdk = { git = "https://github.com/bsundman/nodle-plugin-sdk" }

[lib]
crate-type = ["cdylib"]

# Optional: Optimize for smaller binary size
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

### Cross-Platform Building
```bash
# Windows
cargo build --release --target x86_64-pc-windows-msvc

# macOS
cargo build --release --target x86_64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

## Debugging and Testing

### Debug Output
```rust
impl PluginNode for MyNode {
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        println!("🔍 Processing node with {} inputs", inputs.len());
        
        for (key, value) in inputs {
            match value {
                NodeData::Float(f) => println!("  {}: Float({})", key, f),
                NodeData::String(s) => println!("  {}: String('{}')", key, s),
                NodeData::USDSceneData(usd) => println!("  {}: USD Scene ({} meshes)", key, usd.meshes.len()),
                _ => println!("  {}: {:?}", key, value),
            }
        }
        
        // ... processing logic
    }
}
```

### Error Handling
```rust
impl PluginNode for MyNode {
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        let mut outputs = HashMap::new();
        
        match self.safe_processing(inputs) {
            Ok(result) => {
                outputs.insert("output".to_string(), result);
            },
            Err(e) => {
                eprintln!("Processing error: {}", e);
                outputs.insert("error".to_string(), NodeData::String(format!("Error: {}", e)));
            }
        }
        
        outputs
    }
}
```

## Migration Guide

### From Basic SDK to Modern SDK

1. **Update Data Types**: Replace simple `DataType` usage with rich `NodeData` enum
2. **Add Caching**: Implement `process_with_cache` instead of basic `process`
3. **Enhance UI**: Replace basic parameter controls with `ParameterUI` system
4. **Add Hooks**: Implement `NodeExecutionHooks` for lifecycle management
5. **USD Support**: Use USD data structures for 3D scene processing

### Example Migration
```rust
// Old basic implementation
impl PluginNode for OldNode {
    fn process(&mut self, inputs: &HashMap<String, NodeData>) -> HashMap<String, NodeData> {
        // Simple processing
    }
}

// New modern implementation
impl PluginNode for NewNode {
    fn process_with_cache(
        &mut self, 
        inputs: &HashMap<String, NodeData>,
        cache: &mut dyn PluginCache,
        node_id: u32
    ) -> HashMap<String, NodeData> {
        // Cached processing with multi-stage support
    }
    
    fn get_execution_hooks(&self) -> Option<Box<dyn NodeExecutionHooks>> {
        Some(Box::new(MyNodeHooks))
    }
    
    fn get_parameter_ui(&self) -> ParameterUI {
        // Rich UI with modern components
    }
}
```

This modern SDK provides all the tools needed to create sophisticated, high-performance plugins that integrate seamlessly with Nodle's advanced architecture.