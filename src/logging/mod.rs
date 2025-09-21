use pyo3::prelude::*;
use std::collections::HashMap;

/// Simple logging configuration for feathertail
pub struct LoggingConfig {
    pub level: String,
    pub log_memory: bool,
    pub log_performance: bool,
    pub log_operations: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            log_memory: false,
            log_performance: false,
            log_operations: false,
        }
    }
}

lazy_static::lazy_static! {
    static ref LOGGING_CONFIG: std::sync::Mutex<LoggingConfig> = std::sync::Mutex::new(LoggingConfig::default());
}

/// Initialize logging with default settings
pub fn init_logging() -> PyResult<()> {
    let config = LoggingConfig::default();
    *LOGGING_CONFIG.lock().unwrap() = config;
    Ok(())
}

/// Initialize logging with custom settings
pub fn init_logging_with_config(
    level: &str,
    log_memory: bool,
    log_performance: bool,
    log_operations: bool,
) -> PyResult<()> {
    let config = LoggingConfig {
        level: level.to_string(),
        log_memory,
        log_performance,
        log_operations,
    };
    *LOGGING_CONFIG.lock().unwrap() = config;
    Ok(())
}

/// Check if logging is enabled for the given level
fn should_log(level: &str) -> bool {
    let config = LOGGING_CONFIG.lock().unwrap();
    match config.level.as_str() {
        "trace" => true,
        "debug" => level != "trace",
        "info" => matches!(level, "info" | "warn" | "error"),
        "warn" => matches!(level, "warn" | "error"),
        "error" => level == "error",
        _ => level == "info",
    }
}

/// Log a DataFrame operation
pub fn log_operation(operation: &str, details: &str) {
    if should_log("info") {
        // Use eprintln! for faster output and avoid stdout buffering
        eprintln!("🔧 Operation: {} - {}", operation, details);
    }
}

/// Log memory usage
pub fn log_memory_usage(operation: &str, memory_mb: f64) {
    // Fast path: check if we should log at all before acquiring lock
    if !should_log("info") {
        return;
    }
    
    let config = LOGGING_CONFIG.lock().unwrap();
    if config.log_memory {
        eprintln!("💾 Memory: {} - {:.2} MB", operation, memory_mb);
    }
}

/// Log performance metrics
pub fn log_performance(operation: &str, duration_ms: f64, rows_processed: usize) {
    // Fast path: check if we should log at all before acquiring lock
    if !should_log("info") {
        return;
    }
    
    let config = LOGGING_CONFIG.lock().unwrap();
    if config.log_performance {
        let rows_per_second = if duration_ms > 0.0 {
            (rows_processed as f64 * 1000.0) / duration_ms
        } else {
            0.0
        };
        eprintln!(
            "⚡ Performance: {} - {:.2}ms, {} rows, {:.0} rows/sec",
            operation,
            duration_ms,
            rows_processed,
            rows_per_second
        );
    }
}

/// Log error with context
pub fn log_error(operation: &str, error: &str, context: Option<&str>) {
    if should_log("error") {
        if let Some(ctx) = context {
            println!("❌ Error in {}: {} (Context: {})", operation, error, ctx);
        } else {
            println!("❌ Error in {}: {}", operation, error);
        }
    }
}

/// Log warning with context
pub fn log_warning(operation: &str, warning: &str, context: Option<&str>) {
    if should_log("warn") {
        if let Some(ctx) = context {
            println!("⚠️  Warning in {}: {} (Context: {})", operation, warning, ctx);
        } else {
            println!("⚠️  Warning in {}: {}", operation, warning);
        }
    }
}

/// Log debug information
pub fn log_debug(operation: &str, info: &str) {
    if should_log("debug") {
        println!("🐛 Debug {}: {}", operation, info);
    }
}

/// Log trace information
pub fn log_trace(operation: &str, info: &str) {
    if should_log("trace") {
        println!("🔍 Trace {}: {}", operation, info);
    }
}

/// Get current memory usage in MB (simplified implementation)
pub fn get_memory_usage() -> f64 {
    // This is a simplified implementation
    // In a real implementation, you'd use system-specific APIs
    0.0
}

/// Log DataFrame statistics
pub fn log_dataframe_stats(operation: &str, rows: usize, cols: usize, memory_mb: f64) {
    if should_log("info") {
        println!(
            "📊 DataFrame Stats: {} - {} rows, {} cols, {:.2} MB",
            operation,
            rows,
            cols,
            memory_mb
        );
    }
}

/// Log operation timing
pub fn log_timing(operation: &str, start_time: std::time::Instant) {
    if should_log("debug") {
        let duration = start_time.elapsed();
        println!("⏱️  Timing: {} completed in {:?}", operation, duration);
    }
}

/// Log configuration
pub fn log_config(config: &HashMap<String, String>) {
    if should_log("info") {
        println!("⚙️  Configuration: {:?}", config);
    }
}

/// Log feature usage
pub fn log_feature_usage(feature: &str, usage_count: usize) {
    if should_log("info") {
        println!("🔧 Feature Usage: {} used {} times", feature, usage_count);
    }
}

/// Log performance warning
pub fn log_performance_warning(operation: &str, duration_ms: f64, threshold_ms: f64) {
    if duration_ms > threshold_ms && should_log("warn") {
        println!(
            "⚠️  Performance Warning: {} took {:.2}ms (threshold: {:.2}ms)",
            operation,
            duration_ms,
            threshold_ms
        );
    }
}

/// Log memory warning
pub fn log_memory_warning(operation: &str, memory_mb: f64, threshold_mb: f64) {
    if memory_mb > threshold_mb && should_log("warn") {
        println!(
            "⚠️  Memory Warning: {} used {:.2}MB (threshold: {:.2}MB)",
            operation,
            memory_mb,
            threshold_mb
        );
    }
}