use pyo3::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use std::sync::Arc;

/// Performance profiler for feathertail operations
#[derive(Debug, Clone)]
pub struct Profiler {
    pub name: String,
    pub start_time: Instant,
    pub memory_before: f64,
    pub memory_after: f64,
    pub rows_processed: usize,
    pub columns_processed: usize,
    pub custom_metrics: HashMap<String, f64>,
}

impl Profiler {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start_time: Instant::now(),
            memory_before: 0.0,
            memory_after: 0.0,
            rows_processed: 0,
            columns_processed: 0,
            custom_metrics: HashMap::new(),
        }
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

    pub fn add_metric(&mut self, key: &str, value: f64) {
        self.custom_metrics.insert(key.to_string(), value);
    }

    pub fn get_duration(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn get_duration_ms(&self) -> f64 {
        self.get_duration().as_secs_f64() * 1000.0
    }

    pub fn get_memory_delta(&self) -> f64 {
        self.memory_after - self.memory_before
    }

    pub fn get_rows_per_second(&self) -> f64 {
        let duration_secs = self.get_duration().as_secs_f64();
        if duration_secs > 0.0 {
            self.rows_processed as f64 / duration_secs
        } else {
            0.0
        }
    }

    pub fn get_memory_efficiency(&self) -> f64 {
        if self.rows_processed > 0 {
            self.memory_after / self.rows_processed as f64
        } else {
            0.0
        }
    }

    pub fn get_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("name".to_string(), self.name.clone());
        summary.insert("duration_ms".to_string(), format!("{:.2}", self.get_duration_ms()));
        summary.insert("memory_before_mb".to_string(), format!("{:.2}", self.memory_before));
        summary.insert("memory_after_mb".to_string(), format!("{:.2}", self.memory_after));
        summary.insert("memory_delta_mb".to_string(), format!("{:.2}", self.get_memory_delta()));
        summary.insert("rows_processed".to_string(), self.rows_processed.to_string());
        summary.insert("columns_processed".to_string(), self.columns_processed.to_string());
        summary.insert("rows_per_second".to_string(), format!("{:.0}", self.get_rows_per_second()));
        summary.insert("memory_efficiency".to_string(), format!("{:.4}", self.get_memory_efficiency()));
        
        for (key, value) in &self.custom_metrics {
            summary.insert(key.clone(), format!("{:.2}", value));
        }
        
        summary
    }
}

/// Global profiler state
#[derive(Debug)]
pub struct ProfilerState {
    pub enabled: bool,
    pub profiles: HashMap<String, Vec<Profiler>>,
    pub total_operations: usize,
    pub total_duration_ms: f64,
    pub total_memory_mb: f64,
    pub total_rows_processed: usize,
}

impl Default for ProfilerState {
    fn default() -> Self {
        Self {
            enabled: false,
            profiles: HashMap::new(),
            total_operations: 0,
            total_duration_ms: 0.0,
            total_memory_mb: 0.0,
            total_rows_processed: 0,
        }
    }
}

lazy_static::lazy_static! {
    static ref PROFILER_STATE: Arc<Mutex<ProfilerState>> = Arc::new(Mutex::new(ProfilerState::default()));
}

/// Enable profiling
pub fn enable_profiling() {
    let mut state = PROFILER_STATE.lock().unwrap();
    state.enabled = true;
}

/// Disable profiling
pub fn disable_profiling() {
    let mut state = PROFILER_STATE.lock().unwrap();
    state.enabled = false;
}

/// Check if profiling is enabled
pub fn is_profiling_enabled() -> bool {
    let state = PROFILER_STATE.lock().unwrap();
    state.enabled
}

/// Start profiling an operation
pub fn start_profiling(operation: &str) -> Profiler {
    Profiler::new(operation)
}

/// End profiling and record results
pub fn end_profiling(profiler: Profiler) {
    if !is_profiling_enabled() {
        return;
    }

    let mut state = PROFILER_STATE.lock().unwrap();
    
    // Add to profiles
    state.profiles
        .entry(profiler.name.clone())
        .or_insert_with(Vec::new)
        .push(profiler.clone());
    
    // Update totals
    state.total_operations += 1;
    state.total_duration_ms += profiler.get_duration_ms();
    state.total_memory_mb += profiler.memory_after;
    state.total_rows_processed += profiler.rows_processed;
}

/// Get profiling statistics for an operation
pub fn get_operation_stats(operation: &str) -> Option<HashMap<String, f64>> {
    let state = PROFILER_STATE.lock().unwrap();
    
    if let Some(profiles) = state.profiles.get(operation) {
        if profiles.is_empty() {
            return None;
        }
        
        let mut stats = HashMap::new();
        let count = profiles.len();
        
        // Calculate averages
        let avg_duration = profiles.iter().map(|p| p.get_duration_ms()).sum::<f64>() / count as f64;
        let avg_memory = profiles.iter().map(|p| p.memory_after).sum::<f64>() / count as f64;
        let avg_rows_per_sec = profiles.iter().map(|p| p.get_rows_per_second()).sum::<f64>() / count as f64;
        
        // Calculate min/max
        let min_duration = profiles.iter().map(|p| p.get_duration_ms()).fold(f64::INFINITY, f64::min);
        let max_duration = profiles.iter().map(|p| p.get_duration_ms()).fold(0.0, f64::max);
        
        let min_memory = profiles.iter().map(|p| p.memory_after).fold(f64::INFINITY, f64::min);
        let max_memory = profiles.iter().map(|p| p.memory_after).fold(0.0, f64::max);
        
        stats.insert("count".to_string(), count as f64);
        stats.insert("avg_duration_ms".to_string(), avg_duration);
        stats.insert("min_duration_ms".to_string(), min_duration);
        stats.insert("max_duration_ms".to_string(), max_duration);
        stats.insert("avg_memory_mb".to_string(), avg_memory);
        stats.insert("min_memory_mb".to_string(), min_memory);
        stats.insert("max_memory_mb".to_string(), max_memory);
        stats.insert("avg_rows_per_second".to_string(), avg_rows_per_sec);
        
        Some(stats)
    } else {
        None
    }
}

/// Get overall profiling statistics
pub fn get_overall_stats() -> HashMap<String, f64> {
    let state = PROFILER_STATE.lock().unwrap();
    let mut stats = HashMap::new();
    
    stats.insert("total_operations".to_string(), state.total_operations as f64);
    stats.insert("total_duration_ms".to_string(), state.total_duration_ms);
    stats.insert("total_memory_mb".to_string(), state.total_memory_mb);
    stats.insert("total_rows_processed".to_string(), state.total_rows_processed as f64);
    
    if state.total_operations > 0 {
        stats.insert("avg_duration_ms".to_string(), state.total_duration_ms / state.total_operations as f64);
        stats.insert("avg_memory_mb".to_string(), state.total_memory_mb / state.total_operations as f64);
    }
    
    stats
}

/// Clear profiling data
pub fn clear_profiling_data() {
    let mut state = PROFILER_STATE.lock().unwrap();
    state.profiles.clear();
    state.total_operations = 0;
    state.total_duration_ms = 0.0;
    state.total_memory_mb = 0.0;
    state.total_rows_processed = 0;
}

/// Get profiling report
pub fn get_profiling_report() -> String {
    let state = PROFILER_STATE.lock().unwrap();
    let mut report = String::new();
    
    report.push_str("Feathertail Profiling Report\n");
    report.push_str("============================\n\n");
    
    // Overall statistics
    report.push_str("Overall Statistics:\n");
    report.push_str(&format!("  Total Operations: {}\n", state.total_operations));
    report.push_str(&format!("  Total Duration: {:.2}ms\n", state.total_duration_ms));
    report.push_str(&format!("  Total Memory: {:.2}MB\n", state.total_memory_mb));
    report.push_str(&format!("  Total Rows Processed: {}\n", state.total_rows_processed));
    
    if state.total_operations > 0 {
        report.push_str(&format!("  Average Duration: {:.2}ms\n", state.total_duration_ms / state.total_operations as f64));
        report.push_str(&format!("  Average Memory: {:.2}MB\n", state.total_memory_mb / state.total_operations as f64));
    }
    
    report.push_str("\nOperation Statistics:\n");
    
    // Per-operation statistics
    for (operation, profiles) in &state.profiles {
        if profiles.is_empty() {
            continue;
        }
        
        let count = profiles.len();
        let avg_duration = profiles.iter().map(|p| p.get_duration_ms()).sum::<f64>() / count as f64;
        let avg_memory = profiles.iter().map(|p| p.memory_after).sum::<f64>() / count as f64;
        let avg_rows_per_sec = profiles.iter().map(|p| p.get_rows_per_second()).sum::<f64>() / count as f64;
        
        report.push_str(&format!("  {}:\n", operation));
        report.push_str(&format!("    Count: {}\n", count));
        report.push_str(&format!("    Avg Duration: {:.2}ms\n", avg_duration));
        report.push_str(&format!("    Avg Memory: {:.2}MB\n", avg_memory));
        report.push_str(&format!("    Avg Rows/sec: {:.0}\n", avg_rows_per_sec));
    }
    
    report
}

/// Print profiling report
pub fn print_profiling_report() {
    if is_profiling_enabled() {
        println!("{}", get_profiling_report());
    } else {
        println!("Profiling is disabled. Enable it with enable_profiling()");
    }
}

/// Get memory usage (simplified implementation)
pub fn get_memory_usage() -> f64 {
    // This is a placeholder implementation
    // In a real implementation, you'd use system-specific APIs
    0.0
}

/// Profile a function call
pub fn profile_function<F, R>(name: &str, func: F) -> R
where
    F: FnOnce() -> R,
{
    let mut profiler = start_profiling(name);
    profiler.set_memory_before(get_memory_usage());
    
    let result = func();
    
    profiler.set_memory_after(get_memory_usage());
    end_profiling(profiler);
    
    result
}

/// Profile a function call with custom metrics
pub fn profile_function_with_metrics<F, R>(name: &str, func: F) -> R
where
    F: FnOnce(&mut Profiler) -> R,
{
    let mut profiler = start_profiling(name);
    profiler.set_memory_before(get_memory_usage());
    
    let result = func(&mut profiler);
    
    profiler.set_memory_after(get_memory_usage());
    end_profiling(profiler);
    
    result
}
