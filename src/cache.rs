//! Plugin cache system integration
//!
//! This module provides plugins with access to the main application's
//! unified caching system for performance optimization.

use crate::NodeData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cache key for plugin data that integrates with the main application's cache system
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginCacheKey {
    /// Plugin identifier
    pub plugin_id: String,
    /// Node ID
    pub node_id: u32,
    /// Optional stage identifier for multi-stage operations
    pub stage_id: Option<String>,
    /// Port or data identifier  
    pub port_index: usize,
}

impl PluginCacheKey {
    /// Create a cache key for a single-stage plugin node output
    pub fn new(plugin_id: impl Into<String>, node_id: u32, port_index: usize) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            node_id,
            stage_id: None,
            port_index,
        }
    }
    
    /// Create a cache key for a multi-stage plugin node output
    pub fn with_stage(
        plugin_id: impl Into<String>, 
        node_id: u32, 
        stage_id: impl Into<String>,
        port_index: usize
    ) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            node_id,
            stage_id: Some(stage_id.into()),
            port_index,
        }
    }
    
    /// Check if this is a stage-specific cache key
    pub fn has_stage(&self) -> bool {
        self.stage_id.is_some()
    }
    
    /// Get the stage ID if this is a multi-stage key
    pub fn get_stage(&self) -> Option<&str> {
        self.stage_id.as_deref()
    }
}

/// Pattern for matching cache keys during invalidation
#[derive(Debug, Clone)]
pub enum PluginCacheKeyPattern {
    /// Match all outputs for a specific plugin node
    Node(String, u32), // plugin_id, node_id
    /// Match all outputs for a specific stage of a plugin node  
    Stage(String, u32, String), // plugin_id, node_id, stage_id
    /// Match a specific cache key exactly
    Exact(PluginCacheKey),
    /// Match all cache entries for a plugin
    Plugin(String), // plugin_id
}

impl PluginCacheKeyPattern {
    /// Check if this pattern matches a given cache key
    pub fn matches(&self, key: &PluginCacheKey) -> bool {
        match self {
            PluginCacheKeyPattern::Node(plugin_id, node_id) => {
                key.plugin_id == *plugin_id && key.node_id == *node_id
            },
            PluginCacheKeyPattern::Stage(plugin_id, node_id, stage) => {
                key.plugin_id == *plugin_id 
                    && key.node_id == *node_id 
                    && key.stage_id.as_ref() == Some(stage)
            },
            PluginCacheKeyPattern::Exact(exact_key) => key == exact_key,
            PluginCacheKeyPattern::Plugin(plugin_id) => key.plugin_id == *plugin_id,
        }
    }
}

/// Statistics about plugin cache performance
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PluginCacheStatistics {
    /// Plugin identifier
    pub plugin_id: String,
    /// Total number of cache entries for this plugin
    pub total_entries: usize,
    /// Number of single-stage entries
    pub single_stage_entries: usize,
    /// Number of multi-stage entries
    pub multi_stage_entries: usize,
    /// Cache hits for this plugin
    pub cache_hits: usize,
    /// Cache misses for this plugin
    pub cache_misses: usize,
    /// Cache invalidations for this plugin
    pub cache_invalidations: usize,
    /// Estimated memory usage (in bytes)
    pub estimated_memory_usage: usize,
}

impl PluginCacheStatistics {
    /// Calculate cache hit ratio for this plugin
    pub fn hit_ratio(&self) -> f32 {
        let total_accesses = self.cache_hits + self.cache_misses;
        if total_accesses == 0 {
            0.0
        } else {
            self.cache_hits as f32 / total_accesses as f32
        }
    }
    
    /// Get average memory per entry for this plugin
    pub fn avg_memory_per_entry(&self) -> usize {
        if self.total_entries == 0 {
            0
        } else {
            self.estimated_memory_usage / self.total_entries
        }
    }
}

/// Plugin cache interface
/// 
/// This trait provides plugins with access to the main application's
/// caching system. The main application implements this trait and
/// provides it to plugins during execution.
pub trait PluginCache: Send + Sync {
    /// Store data in the cache
    fn insert(&mut self, key: PluginCacheKey, data: NodeData) -> Result<(), String>;
    
    /// Retrieve data from cache (returns reference)
    fn get(&self, key: &PluginCacheKey) -> Option<&NodeData>;
    
    /// Retrieve and remove data from cache (for move semantics)
    fn take(&mut self, key: &PluginCacheKey) -> Option<NodeData>;
    
    /// Check if a key exists in the cache
    fn contains(&self, key: &PluginCacheKey) -> bool;
    
    /// Invalidate cache entries matching a pattern
    fn invalidate(&mut self, pattern: &PluginCacheKeyPattern) -> usize;
    
    /// Clear all cache entries for a plugin
    fn clear_plugin(&mut self, plugin_id: &str) -> usize;
    
    /// Get cache statistics for a plugin
    fn get_plugin_statistics(&self, plugin_id: &str) -> PluginCacheStatistics;
    
    /// Get all cache keys for a plugin (for debugging/inspection)
    fn get_plugin_keys(&self, plugin_id: &str) -> Vec<&PluginCacheKey>;
}

/// Plugin cache manager
/// 
/// This struct helps plugins manage their cache keys and provides
/// convenient methods for common caching patterns.
#[derive(Debug, Clone)]
pub struct PluginCacheManager {
    /// Plugin identifier
    plugin_id: String,
    /// Currently managed cache keys
    managed_keys: Vec<PluginCacheKey>,
}

impl PluginCacheManager {
    /// Create a new cache manager for a plugin
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            managed_keys: Vec::new(),
        }
    }
    
    /// Create a cache key for this plugin
    pub fn create_key(&self, node_id: u32, port_index: usize) -> PluginCacheKey {
        PluginCacheKey::new(&self.plugin_id, node_id, port_index)
    }
    
    /// Create a stage-specific cache key for this plugin
    pub fn create_stage_key(
        &self, 
        node_id: u32, 
        stage_id: impl Into<String>, 
        port_index: usize
    ) -> PluginCacheKey {
        PluginCacheKey::with_stage(&self.plugin_id, node_id, stage_id, port_index)
    }
    
    /// Store data and track the key
    pub fn store(
        &mut self, 
        cache: &mut dyn PluginCache, 
        key: PluginCacheKey, 
        data: NodeData
    ) -> Result<(), String> {
        let result = cache.insert(key.clone(), data);
        if result.is_ok() {
            self.managed_keys.push(key);
        }
        result
    }
    
    /// Invalidate all cache entries for a specific node
    pub fn invalidate_node(&mut self, cache: &mut dyn PluginCache, node_id: u32) -> usize {
        let pattern = PluginCacheKeyPattern::Node(self.plugin_id.clone(), node_id);
        let invalidated = cache.invalidate(&pattern);
        
        // Remove invalidated keys from managed list
        self.managed_keys.retain(|key| !pattern.matches(key));
        
        invalidated
    }
    
    /// Invalidate all cache entries for a specific stage of a node
    pub fn invalidate_stage(
        &mut self, 
        cache: &mut dyn PluginCache, 
        node_id: u32, 
        stage_id: impl Into<String>
    ) -> usize {
        let stage_id = stage_id.into();
        let pattern = PluginCacheKeyPattern::Stage(self.plugin_id.clone(), node_id, stage_id);
        let invalidated = cache.invalidate(&pattern);
        
        // Remove invalidated keys from managed list
        self.managed_keys.retain(|key| !pattern.matches(key));
        
        invalidated
    }
    
    /// Clear all cache entries for this plugin
    pub fn clear_all(&mut self, cache: &mut dyn PluginCache) -> usize {
        let cleared = cache.clear_plugin(&self.plugin_id);
        self.managed_keys.clear();
        cleared
    }
    
    /// Get cache statistics for this plugin
    pub fn get_statistics(&self, cache: &dyn PluginCache) -> PluginCacheStatistics {
        cache.get_plugin_statistics(&self.plugin_id)
    }
    
    /// Get the plugin ID
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }
    
    /// Get the number of managed cache keys
    pub fn managed_key_count(&self) -> usize {
        self.managed_keys.len()
    }
    
    /// Get all managed cache keys
    pub fn managed_keys(&self) -> &[PluginCacheKey] {
        &self.managed_keys
    }
}

/// Caching strategies for plugins
/// 
/// These provide common patterns that plugins can use for different
/// types of caching scenarios.
pub mod strategies {
    use super::*;
    
    /// Simple single-output caching strategy
    /// 
    /// This is the most basic caching strategy where a plugin caches
    /// the result of processing inputs to avoid recomputation.
    pub struct SimpleCache {
        manager: PluginCacheManager,
    }
    
    impl SimpleCache {
        pub fn new(plugin_id: impl Into<String>) -> Self {
            Self {
                manager: PluginCacheManager::new(plugin_id),
            }
        }
        
        /// Try to get cached result
        pub fn get_cached<'a>(
            &self, 
            cache: &'a dyn PluginCache, 
            node_id: u32, 
            port_index: usize
        ) -> Option<&'a NodeData> {
            let key = self.manager.create_key(node_id, port_index);
            cache.get(&key)
        }
        
        /// Store result in cache
        pub fn store_result(
            &mut self, 
            cache: &mut dyn PluginCache, 
            node_id: u32, 
            port_index: usize, 
            data: NodeData
        ) -> Result<(), String> {
            let key = self.manager.create_key(node_id, port_index);
            self.manager.store(cache, key, data)
        }
        
        /// Invalidate cache for a node
        pub fn invalidate(&mut self, cache: &mut dyn PluginCache, node_id: u32) -> usize {
            self.manager.invalidate_node(cache, node_id)
        }
    }
    
    /// Multi-stage caching strategy (like USD File Reader)
    /// 
    /// This strategy supports multiple stages of processing where each
    /// stage can be cached independently. Stage 1 might load data from
    /// disk, Stage 2 might process it, etc.
    pub struct MultiStageCache {
        manager: PluginCacheManager,
    }
    
    impl MultiStageCache {
        pub fn new(plugin_id: impl Into<String>) -> Self {
            Self {
                manager: PluginCacheManager::new(plugin_id),
            }
        }
        
        /// Try to get cached result for a specific stage
        pub fn get_stage_cached<'a>(
            &self, 
            cache: &'a dyn PluginCache, 
            node_id: u32, 
            stage_id: impl Into<String>,
            port_index: usize
        ) -> Option<&'a NodeData> {
            let key = self.manager.create_stage_key(node_id, stage_id, port_index);
            cache.get(&key)
        }
        
        /// Store result for a specific stage
        pub fn store_stage_result(
            &mut self, 
            cache: &mut dyn PluginCache, 
            node_id: u32, 
            stage_id: impl Into<String>,
            port_index: usize, 
            data: NodeData
        ) -> Result<(), String> {
            let key = self.manager.create_stage_key(node_id, stage_id, port_index);
            self.manager.store(cache, key, data)
        }
        
        /// Invalidate cache for a specific stage
        pub fn invalidate_stage(
            &mut self, 
            cache: &mut dyn PluginCache, 
            node_id: u32,
            stage_id: impl Into<String>
        ) -> usize {
            self.manager.invalidate_stage(cache, node_id, stage_id)
        }
        
        /// Invalidate all stages for a node
        pub fn invalidate_all_stages(&mut self, cache: &mut dyn PluginCache, node_id: u32) -> usize {
            self.manager.invalidate_node(cache, node_id)
        }
    }
}

/// Example usage of the plugin cache system
/// 
/// This shows how plugins can use the cache system for performance optimization.
pub mod examples {
    use super::*;
    use super::strategies::*;
    
    /// Example plugin that uses simple caching
    pub struct ExampleSimpleCachedPlugin {
        cache_strategy: SimpleCache,
    }
    
    impl ExampleSimpleCachedPlugin {
        pub fn new() -> Self {
            Self {
                cache_strategy: SimpleCache::new("example_simple_plugin"),
            }
        }
        
        pub fn process_with_cache(
            &mut self,
            cache: &mut dyn PluginCache,
            node_id: u32,
            inputs: &HashMap<String, NodeData>
        ) -> Result<HashMap<String, NodeData>, String> {
            // Try to get cached result first
            if let Some(cached_result) = self.cache_strategy.get_cached(cache, node_id, 0) {
                println!("🔥 Cache hit for node {}", node_id);
                let mut outputs = HashMap::new();
                outputs.insert("output".to_string(), cached_result.clone());
                return Ok(outputs);
            }
            
            // Cache miss - perform expensive computation
            println!("💾 Cache miss for node {} - computing", node_id);
            
            // Simulate expensive processing
            let result = self.expensive_computation(inputs)?;
            
            // Store result in cache
            self.cache_strategy.store_result(cache, node_id, 0, result.clone())?;
            
            let mut outputs = HashMap::new();
            outputs.insert("output".to_string(), result);
            Ok(outputs)
        }
        
        fn expensive_computation(&self, _inputs: &HashMap<String, NodeData>) -> Result<NodeData, String> {
            // Simulate expensive work
            Ok(NodeData::String("Expensive result".to_string()))
        }
    }
    
    /// Example plugin that uses multi-stage caching
    pub struct ExampleMultiStageCachedPlugin {
        cache_strategy: MultiStageCache,
    }
    
    impl ExampleMultiStageCachedPlugin {
        pub fn new() -> Self {
            Self {
                cache_strategy: MultiStageCache::new("example_multistage_plugin"),
            }
        }
        
        pub fn process_with_multistage_cache(
            &mut self,
            cache: &mut dyn PluginCache,
            node_id: u32,
            inputs: &HashMap<String, NodeData>
        ) -> Result<HashMap<String, NodeData>, String> {
            // Stage 1: Load/prepare data
            let stage1_data = if let Some(cached) = self.cache_strategy.get_stage_cached(cache, node_id, "stage1", 0) {
                println!("🔥 Stage 1 cache hit for node {}", node_id);
                cached.clone()
            } else {
                println!("💾 Stage 1 cache miss for node {} - loading", node_id);
                let data = self.stage1_load_data(inputs)?;
                self.cache_strategy.store_stage_result(cache, node_id, "stage1", 0, data.clone())?;
                data
            };
            
            // Stage 2: Process data
            let stage2_data = if let Some(cached) = self.cache_strategy.get_stage_cached(cache, node_id, "stage2", 0) {
                println!("🔥 Stage 2 cache hit for node {}", node_id);
                cached.clone()
            } else {
                println!("💾 Stage 2 cache miss for node {} - processing", node_id);
                let data = self.stage2_process_data(&stage1_data)?;
                self.cache_strategy.store_stage_result(cache, node_id, "stage2", 0, data.clone())?;
                data
            };
            
            let mut outputs = HashMap::new();
            outputs.insert("output".to_string(), stage2_data);
            Ok(outputs)
        }
        
        fn stage1_load_data(&self, _inputs: &HashMap<String, NodeData>) -> Result<NodeData, String> {
            // Simulate loading data (e.g., from file)
            Ok(NodeData::String("Loaded data".to_string()))
        }
        
        fn stage2_process_data(&self, _stage1_data: &NodeData) -> Result<NodeData, String> {
            // Simulate processing loaded data
            Ok(NodeData::String("Processed data".to_string()))
        }
        
        /// Invalidate only Stage 2 when parameters change (keeping Stage 1 intact)
        pub fn on_parameters_changed(&mut self, cache: &mut dyn PluginCache, node_id: u32) {
            println!("🔧 Parameters changed - invalidating Stage 2 cache only");
            self.cache_strategy.invalidate_stage(cache, node_id, "stage2");
        }
        
        /// Invalidate Stage 1 when file path changes (which also invalidates Stage 2)
        pub fn on_file_changed(&mut self, cache: &mut dyn PluginCache, node_id: u32) {
            println!("📁 File changed - invalidating all stages");
            self.cache_strategy.invalidate_all_stages(cache, node_id);
        }
    }
}