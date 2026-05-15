use std::collections::HashMap;
use std::time::Instant;

/// Debug information for DataFrame operations
#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub operation: String,
    pub start_time: Instant,
    pub memory_before: f64,
    pub memory_after: f64,
    pub rows_processed: usize,
    pub columns_processed: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl DebugInfo {
    pub fn new(operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
            start_time: Instant::now(),
            memory_before: 0.0,
            memory_after: 0.0,
            rows_processed: 0,
            columns_processed: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: &str) {
        self.errors.push(error.to_string());
    }

    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }

    pub fn set_memory_before(&mut self, memory: f64) {
        self.memory_before = memory;
    }

    pub fn set_memory_after(&mut self, memory: f64) {
        self.memory_after = memory;
    }

    pub fn set_rows_processed(&mut self, rows: usize) {
        self.rows_processed = rows;
    }

    pub fn set_columns_processed(&mut self, cols: usize) {
        self.columns_processed = cols;
    }

    pub fn get_duration_ms(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64() * 1000.0
    }

    pub fn get_memory_delta(&self) -> f64 {
        self.memory_after - self.memory_before
    }

    pub fn get_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("operation".to_string(), self.operation.clone());
        summary.insert(
            "duration_ms".to_string(),
            format!("{:.2}", self.get_duration_ms()),
        );
        summary.insert(
            "memory_before_mb".to_string(),
            format!("{:.2}", self.memory_before),
        );
        summary.insert(
            "memory_after_mb".to_string(),
            format!("{:.2}", self.memory_after),
        );
        summary.insert(
            "memory_delta_mb".to_string(),
            format!("{:.2}", self.get_memory_delta()),
        );
        summary.insert(
            "rows_processed".to_string(),
            self.rows_processed.to_string(),
        );
        summary.insert(
            "columns_processed".to_string(),
            self.columns_processed.to_string(),
        );
        summary.insert("error_count".to_string(), self.errors.len().to_string());
        summary.insert("warning_count".to_string(), self.warnings.len().to_string());
        summary
    }
}

/// Debug mode configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enabled: bool,
    pub log_operations: bool,
    pub log_memory: bool,
    pub log_performance: bool,
    pub log_errors: bool,
    pub log_warnings: bool,
    pub memory_threshold_mb: f64,
    pub performance_threshold_ms: f64,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            log_operations: true,
            log_memory: true,
            log_performance: true,
            log_errors: true,
            log_warnings: true,
            memory_threshold_mb: 100.0,
            performance_threshold_ms: 1000.0,
        }
    }
}

lazy_static::lazy_static! {
    static ref DEBUG_CONFIG: std::sync::Mutex<DebugConfig> = std::sync::Mutex::new(DebugConfig {
        enabled: false,
        log_operations: true,
        log_memory: true,
        log_performance: true,
        log_errors: true,
        log_warnings: true,
        memory_threshold_mb: 100.0,
        performance_threshold_ms: 1000.0,
    });
}

/// Set debug configuration
pub fn set_debug_config(config: DebugConfig) {
    *DEBUG_CONFIG.lock().unwrap() = config;
}

/// Get debug configuration
pub fn get_debug_config() -> DebugConfig {
    DEBUG_CONFIG.lock().unwrap().clone()
}

/// Enable debug mode
pub fn enable_debug() {
    DEBUG_CONFIG.lock().unwrap().enabled = true;
}

/// Disable debug mode
pub fn disable_debug() {
    DEBUG_CONFIG.lock().unwrap().enabled = false;
}

/// Check if debug mode is enabled
pub fn is_debug_enabled() -> bool {
    DEBUG_CONFIG.lock().unwrap().enabled
}

/// Log debug information if debug mode is enabled
pub fn log_debug_info(info: &DebugInfo) {
    if !is_debug_enabled() {
        return;
    }

    let config = get_debug_config();

    if config.log_operations {
        eprintln!("🔍 Debug Operation: {}", info.operation);
    }

    if config.log_performance {
        let duration = info.get_duration_ms();
        eprintln!("⏱️  Performance: {:.2}ms", duration);

        if duration > config.performance_threshold_ms {
            println!("⚠️  Performance Warning: Operation took longer than threshold");
        }
    }

    if config.log_memory {
        let memory_delta = info.get_memory_delta();
        println!("💾 Memory: {:.2}MB delta", memory_delta);

        if memory_delta.abs() > config.memory_threshold_mb {
            println!("⚠️  Memory Warning: Large memory change detected");
        }
    }

    if config.log_errors && !info.errors.is_empty() {
        println!("❌ Errors: {}", info.errors.join(", "));
    }

    if config.log_warnings && !info.warnings.is_empty() {
        println!("⚠️  Warnings: {}", info.warnings.join(", "));
    }

    println!(
        "📊 Rows: {}, Columns: {}",
        info.rows_processed, info.columns_processed
    );
}

/// Create a debug info for an operation
pub fn create_debug_info(operation: &str) -> DebugInfo {
    DebugInfo::new(operation)
}

/// Log operation start
pub fn log_operation_start(operation: &str) {
    if is_debug_enabled() {
        println!("🚀 Starting operation: {}", operation);
    }
}

/// Log operation end
pub fn log_operation_end(operation: &str, duration_ms: f64) {
    if is_debug_enabled() {
        println!(
            "✅ Completed operation: {} in {:.2}ms",
            operation, duration_ms
        );
    }
}

/// Log memory usage
pub fn log_memory_usage(operation: &str, memory_mb: f64) {
    if is_debug_enabled() {
        println!("💾 Memory usage for {}: {:.2}MB", operation, memory_mb);
    }
}

/// Log performance metrics
pub fn log_performance_metrics(operation: &str, duration_ms: f64, rows_per_second: f64) {
    if is_debug_enabled() {
        println!(
            "⚡ Performance for {}: {:.2}ms, {:.0} rows/sec",
            operation, duration_ms, rows_per_second
        );
    }
}

/// Log error with context
pub fn log_debug_error(operation: &str, error: &str, context: Option<&str>) {
    if is_debug_enabled() {
        if let Some(ctx) = context {
            println!(
                "❌ Debug Error in {}: {} (Context: {})",
                operation, error, ctx
            );
        } else {
            println!("❌ Debug Error in {}: {}", operation, error);
        }
    }
}

/// Log warning with context
pub fn log_debug_warning(operation: &str, warning: &str, context: Option<&str>) {
    if is_debug_enabled() {
        if let Some(ctx) = context {
            println!(
                "⚠️  Debug Warning in {}: {} (Context: {})",
                operation, warning, ctx
            );
        } else {
            println!("⚠️  Debug Warning in {}: {}", operation, warning);
        }
    }
}

/// Log DataFrame information
pub fn log_dataframe_info(operation: &str, rows: usize, cols: usize, memory_mb: f64) {
    if is_debug_enabled() {
        println!(
            "📊 DataFrame Info for {}: {} rows, {} cols, {:.2}MB",
            operation, rows, cols, memory_mb
        );
    }
}

/// Log column information
pub fn log_column_info(operation: &str, column: &str, dtype: &str, null_count: usize) {
    if is_debug_enabled() {
        println!(
            "📋 Column Info for {}: {} ({}, {} nulls)",
            operation, column, dtype, null_count
        );
    }
}

/// Log operation statistics
pub fn log_operation_stats(operation: &str, stats: &HashMap<String, String>) {
    if is_debug_enabled() {
        println!("📈 Operation Stats for {}:", operation);
        for (key, value) in stats {
            println!("  {}: {}", key, value);
        }
    }
}

/// Get current memory usage (simplified implementation)
pub fn get_current_memory_usage() -> f64 {
    // This is a placeholder implementation
    // In a real implementation, you'd use system-specific APIs
    0.0
}

/// Log feature usage
pub fn log_feature_usage(feature: &str, usage_count: usize) {
    if is_debug_enabled() {
        println!("🔧 Feature Usage: {} used {} times", feature, usage_count);
    }
}

/// Log configuration changes
pub fn log_config_change(key: &str, old_value: &str, new_value: &str) {
    if is_debug_enabled() {
        println!(
            "⚙️  Config Change: {} changed from '{}' to '{}'",
            key, old_value, new_value
        );
    }
}

/// Log system information
pub fn log_system_info() {
    if is_debug_enabled() {
        println!("🖥️  System Info:");
        println!("  Rust version: {}", env!("CARGO_PKG_VERSION"));
        println!(
            "  Target: {}",
            std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string())
        );
        println!(
            "  Debug mode: {}",
            if cfg!(debug_assertions) { "Yes" } else { "No" }
        );
    }
}
