# Nodle Plugin SDK

The official Plugin SDK for [Nodle](https://github.com/bsundman/nodle) - a node-based visual programming editor.

This SDK provides the core interfaces and types needed to create dynamic plugins that can be loaded at runtime into Nodle.

## Features

### Core Capabilities
- **Type-safe plugin interfaces** - Well-defined traits for plugins and nodes
- **Rich metadata system** - Comprehensive node descriptions and categorization  
- **Cross-platform support** - Works on Windows, macOS, and Linux
- **Runtime loading** - Dynamic library loading with hot-swapping
- **Integration ready** - Seamless integration with Nodle's workspace system

### Advanced Features (2024 Update)
- **Complete NodeData System** - Full compatibility with Scene, Geometry, Material, USD, Light, and Image data types
- **Multi-Stage Caching** - Intelligent caching system with stage-qualified keys for maximum performance
- **Execution Hooks** - Complete lifecycle management with parameter change notifications
- **Rich UI Components** - Modern parameter interfaces with sliders, color pickers, file dialogs, and more
- **USD Integration** - Native support for USD scene data processing and manipulation
- **Performance Optimization** - Direct access to Nodle's unified cache system

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
nodle-plugin-sdk = { git = "https://github.com/bsundman/nodle-plugin-sdk" }

[lib]
crate-type = ["cdylib"]
```

Create a basic plugin:

```rust
use nodle_plugin_sdk::*;
use std::collections::HashMap;

pub struct MyPlugin;

impl NodePlugin for MyPlugin {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "My Plugin".to_string(),
            version: "0.1.0".to_string(),
            author: "Your Name".to_string(),
            description: "My awesome plugin".to_string(),
            compatible_version: "0.1.0".to_string(),
        }
    }
    
    fn register_nodes(&self, registry: &mut dyn NodeRegistryTrait) {
        registry.register_node_factory(Box::new(MyNodeFactory)).unwrap();
    }
    
    fn on_load(&self) -> Result<(), PluginError> {
        println!("Plugin loaded!");
        Ok(())
    }
    
    fn on_unload(&self) -> Result<(), PluginError> {
        println!("Plugin unloaded");
        Ok(())
    }
}

// Export required C functions
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn NodePlugin {
    Box::into_raw(Box::new(MyPlugin))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn NodePlugin) {
    unsafe { let _ = Box::from_raw(plugin); }
}
```

## Core Concepts

### Plugin Lifecycle

1. **Loading** - Nodle discovers and loads your `.dll`/`.dylib`/`.so` file
2. **Registration** - Your plugin registers node factories via `register_nodes()`
3. **Runtime** - Users create instances of your nodes in Nodle
4. **Unloading** - Clean shutdown when Nodle exits or plugin is removed

### Node Architecture

- **NodeFactory** - Creates instances of your node type
- **PluginNode** - The actual node implementation with parameters and processing
- **NodeMetadata** - Rich description of your node's capabilities

### Data Flow

- **Inputs/Outputs** - Strongly typed ports for connecting nodes with rich data types
- **Parameters** - User-configurable properties with modern UI components
- **Processing** - Transform input data to output data with optional caching
- **USD Integration** - Native support for USD scene data, meshes, lights, and materials
- **Caching** - Multi-stage caching for high-performance file loading and processing

## API Reference

### Core Traits

- [`NodePlugin`](src/plugin.rs) - Main plugin interface with lifecycle management
- [`NodeFactory`](src/plugin.rs) - Factory for creating node instances  
- [`PluginNode`](src/plugin.rs) - Individual node implementation with caching support
- [`NodeExecutionHooks`](src/hooks.rs) - Lifecycle hooks for advanced cache management
- [`PluginCache`](src/cache.rs) - Unified caching interface for performance optimization

### Data Systems

- [`NodeData`](src/data_types.rs) - Complete data type system with USD, Scene, Geometry, Material support
- [`InterfaceParameter`](src/ui.rs) - Rich parameter system for modern UI components
- [`ParameterUI`](src/ui.rs) - UI builder system for complex parameter interfaces

### Advanced Features

- [`MultiStageCache`](src/cache.rs) - Multi-stage caching strategy for complex operations
- [`SimpleCache`](src/cache.rs) - Basic caching strategy for simple operations  
- [`USDSceneData`](src/data_types.rs) - Complete USD scene data structures
- [`NodeRegistryTrait`](src/registry.rs) - Registry for node factories

### Data Types

- [`NodeData`](src/data_types.rs) - Variant type for node parameters and I/O
- [`NodeMetadata`](src/metadata.rs) - Comprehensive node descriptions
- [`PluginInfo`](src/plugin.rs) - Plugin identification and versioning

### Error Handling

- [`PluginError`](src/errors.rs) - Standard error types for plugin operations

## Examples

See the [plugin template](https://github.com/bsundman/nodle-plugin-template) for a complete working example with:

- Hello World node with editable text
- Math Add node with numeric inputs/outputs  
- Proper error handling and resource management

## Building Plugins

1. **Develop** your plugin using this SDK
2. **Build** as a dynamic library: `cargo build --release`
3. **Install** by copying to Nodle's plugins directory
4. **Test** by launching Nodle - your nodes appear in the menus

## Platform-Specific Notes

### macOS
- Plugins are `.dylib` files
- May need to sign plugins for distribution

### Windows  
- Plugins are `.dll` files
- Ensure Visual C++ runtime compatibility

### Linux
- Plugins are `.so` files
- Check `glibc` version compatibility

## Contributing

We welcome contributions! Please:

1. Fork this repository
2. Create a feature branch
3. Add tests for new functionality  
4. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- [Nodle](https://github.com/bsundman/nodle) - The main node editor
- [Plugin Template](https://github.com/bsundman/nodle-plugin-template) - Starter template for plugins