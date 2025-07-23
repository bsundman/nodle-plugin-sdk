//! Node execution hooks for cache clearing and preparation
//!
//! This module provides a trait-based system for plugins to handle their own
//! cache clearing and resource management during the execution lifecycle.

use crate::{NodeData, PluginHandle};
use std::collections::HashMap;

/// Trait for node-specific execution lifecycle hooks
/// 
/// Plugin nodes can implement this trait to participate in the advanced cache management
/// and lifecycle systems that core nodes use. This allows plugins to:
/// - Clear their own caches before execution
/// - Handle resource cleanup when nodes are removed
/// - React to connection changes for cache invalidation
/// - Perform custom setup/teardown operations
pub trait NodeExecutionHooks: Send + Sync {
    /// Called before node execution - handle cache clearing and preparation
    /// 
    /// This is where plugins should clear any internal caches, validate inputs,
    /// and prepare for execution. The node and its connections are provided
    /// for context.
    /// 
    /// # Arguments
    /// * `plugin_handle` - Handle to the plugin instance
    /// * `node_id` - ID of the node being executed
    /// * `connections` - Current input connections and their data
    /// 
    /// # Returns
    /// * `Ok(())` if preparation succeeded
    /// * `Err(String)` with error message if preparation failed
    fn before_execution(
        &mut self, 
        plugin_handle: &PluginHandle,
        node_id: u32,
        connections: &HashMap<String, NodeData>
    ) -> Result<(), String> {
        // Default: no special handling
        Ok(())
    }
    
    /// Called after successful node execution - handle caching and cleanup
    /// 
    /// This is where plugins can cache results, update internal state,
    /// or perform cleanup operations after successful execution.
    /// 
    /// # Arguments
    /// * `plugin_handle` - Handle to the plugin instance
    /// * `node_id` - ID of the node that was executed
    /// * `outputs` - The output data produced by execution
    /// 
    /// # Returns
    /// * `Ok(())` if post-processing succeeded
    /// * `Err(String)` with error message if post-processing failed
    fn after_execution(
        &mut self, 
        plugin_handle: &PluginHandle,
        node_id: u32,
        outputs: &HashMap<String, NodeData>
    ) -> Result<(), String> {
        // Default: no special handling
        Ok(())
    }
    
    /// Called when node is removed from graph - handle cleanup
    /// 
    /// This is where plugins should clean up any resources, caches, or
    /// persistent state associated with the removed node.
    /// 
    /// # Arguments
    /// * `plugin_handle` - Handle to the plugin instance
    /// * `node_id` - ID of the node being removed
    /// 
    /// # Returns
    /// * `Ok(())` if cleanup succeeded
    /// * `Err(String)` with error message if cleanup failed
    fn on_node_removed(
        &mut self, 
        plugin_handle: &PluginHandle,
        node_id: u32
    ) -> Result<(), String> {
        // Default: no special handling
        Ok(())
    }
    
    /// Called when a connection is added TO this node (this node receives new input)
    /// 
    /// This allows plugins to invalidate caches or update internal state when
    /// their inputs change. This is called for each new input connection.
    /// 
    /// # Arguments
    /// * `plugin_handle` - Handle to the plugin instance
    /// * `node_id` - ID of the node receiving the new connection
    /// * `input_port` - Name/ID of the input port being connected
    /// * `source_node_id` - ID of the node providing the input
    /// 
    /// # Returns
    /// * `Ok(())` if handling succeeded
    /// * `Err(String)` with error message if handling failed
    fn on_input_connection_added(
        &mut self, 
        plugin_handle: &PluginHandle,
        node_id: u32,
        input_port: &str,
        source_node_id: u32
    ) -> Result<(), String> {
        // Default: no special handling
        Ok(())
    }
    
    /// Called when a connection is removed FROM this node (this node loses an input)
    /// 
    /// This allows plugins to clean up caches or update internal state when
    /// their inputs are disconnected. This is called for each removed input connection.
    /// 
    /// # Arguments
    /// * `plugin_handle` - Handle to the plugin instance
    /// * `node_id` - ID of the node losing the connection
    /// * `input_port` - Name/ID of the input port being disconnected
    /// * `source_node_id` - ID of the node that was providing the input
    /// 
    /// # Returns
    /// * `Ok(())` if handling succeeded
    /// * `Err(String)` with error message if handling failed
    fn on_input_connection_removed(
        &mut self, 
        plugin_handle: &PluginHandle,
        node_id: u32,
        input_port: &str,
        source_node_id: u32
    ) -> Result<(), String> {
        // Default: no special handling
        Ok(())
    }
    
    /// Called when node parameters change
    /// 
    /// This allows plugins to invalidate caches or update internal state when
    /// node parameters are modified through the UI or programmatically.
    /// 
    /// # Arguments
    /// * `plugin_handle` - Handle to the plugin instance
    /// * `node_id` - ID of the node whose parameters changed
    /// * `parameter_name` - Name of the parameter that changed
    /// * `old_value` - Previous value of the parameter
    /// * `new_value` - New value of the parameter
    /// 
    /// # Returns
    /// * `Ok(())` if handling succeeded
    /// * `Err(String)` with error message if handling failed
    fn on_parameter_changed(
        &mut self,
        plugin_handle: &PluginHandle,
        node_id: u32,
        parameter_name: &str,
        old_value: &NodeData,
        new_value: &NodeData
    ) -> Result<(), String> {
        // Default: no special handling
        Ok(())
    }
    
    /// Clone the hooks for registration
    /// 
    /// This is required for the plugin system to manage hook instances.
    /// Plugins should return a new boxed instance of their hooks implementation.
    fn clone_box(&self) -> Box<dyn NodeExecutionHooks>;
}

/// Default implementation for nodes that don't need special handling
/// 
/// Plugins that don't need lifecycle hooks can use this default implementation
/// or implement the trait with empty methods.
#[derive(Clone)]
pub struct DefaultHooks;

impl NodeExecutionHooks for DefaultHooks {
    fn clone_box(&self) -> Box<dyn NodeExecutionHooks> {
        Box::new(self.clone())
    }
}

/// Hook registration information
/// 
/// This struct is used to register hooks with the main application's execution engine.
/// Plugins provide this when they want to participate in lifecycle management.
#[derive(Debug, Clone)]
pub struct HookRegistration {
    /// The node type ID that these hooks apply to
    pub node_type_id: String,
    /// Description of what these hooks do (for debugging)
    pub description: String,
}

impl HookRegistration {
    /// Create a new hook registration
    pub fn new(node_type_id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            node_type_id: node_type_id.into(),
            description: description.into(),
        }
    }
}

/// Cache management utilities for plugins
/// 
/// These utilities help plugins work with the main application's cache system
/// without exposing the full complexity of the internal cache implementation.
pub mod cache_utils {
    use super::*;
    
    /// Cache key for plugin data
    /// 
    /// Plugins can use this to create consistent cache keys for their data.
    /// The main application will handle the actual caching mechanics.
    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    pub struct PluginCacheKey {
        /// Plugin identifier
        pub plugin_id: String,
        /// Node ID
        pub node_id: u32,
        /// Optional stage identifier for multi-stage operations
        pub stage_id: Option<String>,
        /// Port or data identifier
        pub data_id: String,
    }
    
    impl PluginCacheKey {
        /// Create a simple cache key for single-stage operations
        pub fn simple(plugin_id: impl Into<String>, node_id: u32, data_id: impl Into<String>) -> Self {
            Self {
                plugin_id: plugin_id.into(),
                node_id,
                stage_id: None,
                data_id: data_id.into(),
            }
        }
        
        /// Create a cache key for multi-stage operations (like USD File Reader)
        pub fn with_stage(
            plugin_id: impl Into<String>, 
            node_id: u32, 
            stage_id: impl Into<String>,
            data_id: impl Into<String>
        ) -> Self {
            Self {
                plugin_id: plugin_id.into(),
                node_id,
                stage_id: Some(stage_id.into()),
                data_id: data_id.into(),
            }
        }
        
        /// Convert to string representation for use with main application cache
        pub fn to_cache_string(&self) -> String {
            match &self.stage_id {
                Some(stage) => format!("plugin:{}:{}:{}:{}", self.plugin_id, self.node_id, stage, self.data_id),
                None => format!("plugin:{}:{}:{}", self.plugin_id, self.node_id, self.data_id),
            }
        }
    }
    
    /// Cache invalidation patterns for plugins
    /// 
    /// These patterns help plugins specify what should be invalidated
    /// when their state changes.
    #[derive(Debug, Clone)]
    pub enum CacheInvalidationPattern {
        /// Invalidate all cache entries for a specific node
        AllForNode(u32),
        /// Invalidate all cache entries for a specific stage of a node
        StageForNode(u32, String),
        /// Invalidate a specific cache entry
        Specific(PluginCacheKey),
        /// Invalidate all cache entries for this plugin
        AllForPlugin(String),
    }
}

/// Example hook implementation for plugins that need advanced caching
/// 
/// This shows how plugins can implement sophisticated cache management
/// similar to what core nodes like USD File Reader use.
#[derive(Clone)]
pub struct ExampleAdvancedHooks {
    /// Plugin identifier
    plugin_id: String,
    /// Cache keys currently managed by this plugin
    managed_keys: Vec<cache_utils::PluginCacheKey>,
}

impl ExampleAdvancedHooks {
    /// Create new advanced hooks for a plugin
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            managed_keys: Vec::new(),
        }
    }
}

impl NodeExecutionHooks for ExampleAdvancedHooks {
    fn before_execution(
        &mut self, 
        _plugin_handle: &PluginHandle,
        node_id: u32,
        _connections: &HashMap<String, NodeData>
    ) -> Result<(), String> {
        // Example: Clear any temporary caches before execution
        println!("🔧 Plugin {}: Preparing node {} for execution", self.plugin_id, node_id);
        
        // In a real implementation, you might:
        // - Validate input data
        // - Clear temporary caches
        // - Set up resources needed for execution
        // - Check for parameter changes that require cache invalidation
        
        Ok(())
    }
    
    fn after_execution(
        &mut self, 
        _plugin_handle: &PluginHandle,
        node_id: u32,
        outputs: &HashMap<String, NodeData>
    ) -> Result<(), String> {
        // Example: Cache results after successful execution
        println!("🔧 Plugin {}: Caching results for node {} ({} outputs)", 
                 self.plugin_id, node_id, outputs.len());
        
        // In a real implementation, you might:
        // - Cache expensive computation results
        // - Update internal state based on outputs
        // - Trigger dependent operations
        // - Update statistics or metrics
        
        Ok(())
    }
    
    fn on_input_connection_added(
        &mut self, 
        _plugin_handle: &PluginHandle,
        node_id: u32,
        input_port: &str,
        source_node_id: u32
    ) -> Result<(), String> {
        // Example: Invalidate caches when inputs change
        println!("🔗 Plugin {}: New connection to node {} port {} from node {}", 
                 self.plugin_id, node_id, input_port, source_node_id);
        
        // In a real implementation, you might:
        // - Invalidate caches that depend on this input
        // - Update internal dependency tracking
        // - Validate that the new connection is compatible
        
        Ok(())
    }
    
    fn on_parameter_changed(
        &mut self,
        _plugin_handle: &PluginHandle,
        node_id: u32,
        parameter_name: &str,
        _old_value: &NodeData,
        _new_value: &NodeData
    ) -> Result<(), String> {
        // Example: Selective cache invalidation based on parameter
        println!("🔧 Plugin {}: Parameter '{}' changed on node {}", 
                 self.plugin_id, parameter_name, node_id);
        
        // In a real implementation, you might:
        // - Only invalidate caches affected by this specific parameter
        // - Implement multi-stage caching (like USD File Reader)
        // - Update internal state to reflect the parameter change
        
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn NodeExecutionHooks> {
        Box::new(self.clone())
    }
}