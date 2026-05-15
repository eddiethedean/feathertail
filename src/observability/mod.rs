//! Facade over logging (and hooks for tests / future sinks). Python entrypoints in [`crate`] call
//! into this layer so callers depend on [`ObservabilitySink`] rather than the logging module wire-up.

use std::sync::{Arc, RwLock};

lazy_static::lazy_static! {
    static ref OBSERVABILITY: RwLock<Option<Arc<dyn ObservabilitySink + Send + Sync>>> =
        RwLock::new(None);
}

/// Injectable sink for dataframe operation telemetry (DIP). Default resolves to [`LoggingSink`].
pub trait ObservabilitySink: Send + Sync {
    fn log_operation(&self, operation: &str, details: &str);
    fn log_memory_usage(&self, operation: &str, memory_mb: f64);
    fn log_performance(&self, operation: &str, duration_ms: f64, rows_processed: usize);
    fn log_error(&self, operation: &str, error: &str, context: Option<&str>);
    fn log_warning(&self, operation: &str, warning: &str, context: Option<&str>);
}

struct LoggingSink;

impl ObservabilitySink for LoggingSink {
    fn log_operation(&self, operation: &str, details: &str) {
        crate::logging::log_operation(operation, details);
    }
    fn log_memory_usage(&self, operation: &str, memory_mb: f64) {
        crate::logging::log_memory_usage(operation, memory_mb);
    }
    fn log_performance(&self, operation: &str, duration_ms: f64, rows_processed: usize) {
        crate::logging::log_performance(operation, duration_ms, rows_processed);
    }
    fn log_error(&self, operation: &str, error: &str, context: Option<&str>) {
        crate::logging::log_error(operation, error, context);
    }
    fn log_warning(&self, operation: &str, warning: &str, context: Option<&str>) {
        crate::logging::log_warning(operation, warning, context);
    }
}

fn active_sink() -> Arc<dyn ObservabilitySink + Send + Sync> {
    let g = OBSERVABILITY.read().expect("observability lock poisoned");
    g.clone().unwrap_or_else(|| Arc::new(LoggingSink))
}

/// Override the global sink (e.g. tests). Passing `None` restores the built-in logging bridge.
pub fn set_observability_sink(sink: Option<Arc<dyn ObservabilitySink + Send + Sync>>) {
    let mut g = OBSERVABILITY.write().expect("observability lock poisoned");
    *g = sink;
}

pub fn log_operation(operation: &str, details: &str) {
    active_sink().log_operation(operation, details);
}

pub fn log_memory_usage(operation: &str, memory_mb: f64) {
    active_sink().log_memory_usage(operation, memory_mb);
}

pub fn log_performance(operation: &str, duration_ms: f64, rows_processed: usize) {
    active_sink().log_performance(operation, duration_ms, rows_processed);
}

pub fn log_error(operation: &str, error: &str, context: Option<&str>) {
    active_sink().log_error(operation, error, context);
}

pub fn log_warning(operation: &str, warning: &str, context: Option<&str>) {
    active_sink().log_warning(operation, warning, context);
}
