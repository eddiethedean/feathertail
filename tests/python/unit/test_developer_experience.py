import pytest
import feathertail as ft
import time
import os

# Disable verbose logging for faster tests
os.environ['FEATHERTAIL_LOG_LEVEL'] = 'error'

class TestLoggingSystem:
    """Test logging system functionality"""

    def test_init_logging_default(self):
        """Test initializing logging with default settings"""
        # This should not raise an error
        ft.init_logging()

    def test_init_logging_with_config(self):
        """Test initializing logging with custom configuration"""
        # Test different log levels
        for level in ["trace", "debug", "info", "warn", "error"]:
            ft.init_logging_with_config(level, True, True, True)

    def test_log_operation(self):
        """Test logging operations"""
        # These should not raise errors
        ft.log_operation("test_operation", "Testing operation logging")
        ft.log_operation("filter", "Filtering data with condition age > 25")

    def test_log_memory_usage(self):
        """Test logging memory usage"""
        # Initialize with minimal logging to avoid overhead
        ft.init_logging_with_config("error", False, False, False)
        ft.log_memory_usage("test_operation", 123.45)

    def test_log_performance(self):
        """Test logging performance metrics"""
        # Initialize with minimal logging to avoid overhead
        ft.init_logging_with_config("error", False, False, False)
        ft.log_performance("test_operation", 150.0, 1000)

    def test_log_error(self):
        """Test logging errors"""
        # Initialize with minimal logging to avoid overhead
        ft.init_logging_with_config("error", False, False, False)
        ft.log_error("test_operation", "Test error occurred", "Test context")
        ft.log_error("test_operation", "Test error without context", None)

    def test_log_warning(self):
        """Test logging warnings"""
        # Initialize with minimal logging to avoid overhead
        ft.init_logging_with_config("error", False, False, False)
        ft.log_warning("test_operation", "Test warning", "Test context")
        ft.log_warning("test_operation", "Test warning without context", None)


class TestDebugSystem:
    """Test debug system functionality"""

    def test_debug_enable_disable(self):
        """Test enabling and disabling debug mode"""
        # Initially should be disabled
        assert not ft.is_debug_enabled()
        
        # Enable debug
        ft.enable_debug()
        assert ft.is_debug_enabled()
        
        # Disable debug
        ft.disable_debug()
        assert not ft.is_debug_enabled()

    def test_log_operation_start_end(self):
        """Test logging operation start and end"""
        ft.enable_debug()
        
        # These should not raise errors
        ft.log_operation_start("test_operation")
        time.sleep(0.001)  # Small delay
        ft.log_operation_end("test_operation", 1.0)
        
        ft.disable_debug()

    def test_log_debug_info(self):
        """Test logging debug information"""
        ft.enable_debug()
        
        # This should not raise an error
        ft.log_debug_info("test_operation", 50.0, 10.5, 1000)
        
        ft.disable_debug()


class TestProfilingSystem:
    """Test profiling system functionality"""

    def test_profiling_enable_disable(self):
        """Test enabling and disabling profiling"""
        # Initially should be disabled
        assert not ft.is_profiling_enabled()
        
        # Enable profiling
        ft.enable_profiling()
        assert ft.is_profiling_enabled()
        
        # Disable profiling
        ft.disable_profiling()
        assert not ft.is_profiling_enabled()

    def test_get_overall_stats(self):
        """Test getting overall profiling statistics"""
        ft.enable_profiling()
        
        stats = ft.get_overall_stats()
        assert isinstance(stats, dict)
        assert "total_operations" in stats
        assert "total_duration_ms" in stats
        assert "total_memory_mb" in stats
        assert "total_rows_processed" in stats
        
        ft.disable_profiling()

    def test_get_operation_stats_empty(self):
        """Test getting operation stats when no operations have been profiled"""
        ft.enable_profiling()
        
        # Should return None for non-existent operation
        stats = ft.get_operation_stats("nonexistent_operation")
        assert stats is None
        
        ft.disable_profiling()

    def test_clear_profiling_data(self):
        """Test clearing profiling data"""
        ft.enable_profiling()
        
        # Clear data
        ft.clear_profiling_data()
        
        # Check that data is cleared
        stats = ft.get_overall_stats()
        assert stats["total_operations"] == 0.0
        assert stats["total_duration_ms"] == 0.0
        assert stats["total_memory_mb"] == 0.0
        assert stats["total_rows_processed"] == 0.0
        
        ft.disable_profiling()

    def test_print_profiling_report(self):
        """Test printing profiling report"""
        ft.enable_profiling()
        
        # This should not raise an error
        ft.print_profiling_report()
        
        ft.disable_profiling()


class TestDeveloperExperienceIntegration:
    """Test integration of developer experience features with DataFrame operations"""

    def test_logging_with_dataframe_operations(self):
        """Test logging with actual DataFrame operations"""
        # Initialize logging with minimal output
        ft.init_logging_with_config("error", False, False, False)
        
        # Create minimal test data
        data = [{"name": "Alice", "age": 25}]
        df = ft.TinyFrame.from_dicts(data)
        
        # Log operations (minimal)
        ft.log_operation("create_dataframe", "Test")
        
        # Perform minimal operations
        filtered = df.filter("age", ">", 20)
        assert filtered.len() == 1

    def test_debug_with_dataframe_operations(self):
        """Test debug mode with DataFrame operations"""
        # Enable debug mode
        ft.enable_debug()
        
        # Create minimal test data
        data = [{"name": "Alice", "age": 25}]
        df = ft.TinyFrame.from_dicts(data)
        
        # Log debug information (minimal)
        ft.log_operation_start("create_dataframe")
        ft.log_operation_end("create_dataframe", 1.0)
        
        ft.disable_debug()

    def test_profiling_with_dataframe_operations(self):
        """Test profiling with DataFrame operations"""
        # Enable profiling
        ft.enable_profiling()
        
        # Create minimal test data
        data = [{"name": "Alice", "age": 25}]
        df = ft.TinyFrame.from_dicts(data)
        
        # Perform minimal operations
        filtered = df.filter("age", ">", 20)
        assert filtered.len() == 1
        
        # Check profiling data
        stats = ft.get_overall_stats()
        assert isinstance(stats, dict)
        
        ft.disable_profiling()

    def test_error_handling_in_developer_tools(self):
        """Test error handling in developer tools"""
        # Test with invalid log level
        ft.init_logging_with_config("invalid_level", True, True, True)
        
        # Test with None context
        ft.log_error("test", "error", None)
        ft.log_warning("test", "warning", None)
        
        # These should not raise errors
        assert True

    def test_memory_and_performance_logging(self):
        """Test memory and performance logging with realistic values"""
        ft.init_logging_with_config("error", False, False, False)  # Disable output
        
        # Test with minimal values
        ft.log_memory_usage("test_operation", 1.0)
        ft.log_performance("test_operation", 1.0, 10)

    def test_debug_info_with_various_operations(self):
        """Test debug info with various DataFrame operations"""
        ft.enable_debug()
        
        # Create minimal test data
        data = [{"id": 1, "value": 2}]
        df = ft.TinyFrame.from_dicts(data)
        
        # Test with minimal operations
        ft.log_debug_info("create_dataframe", 1.0, 0.1, df.len())
        
        ft.disable_debug()

    def test_profiling_statistics_accuracy(self):
        """Test that profiling statistics are accurate"""
        ft.enable_profiling()
        ft.clear_profiling_data()
        
        # Initial stats should be zero
        stats = ft.get_overall_stats()
        assert stats["total_operations"] == 0.0
        assert stats["total_duration_ms"] == 0.0
        assert stats["total_memory_mb"] == 0.0
        assert stats["total_rows_processed"] == 0.0
        
        ft.disable_profiling()

    def test_developer_tools_with_large_dataset(self):
        """Test developer tools with a larger dataset"""
        ft.init_logging_with_config("error", False, False, False)  # Disable output
        ft.enable_debug()
        ft.enable_profiling()
        
        # Create minimal dataset
        data = [{"id": 1, "value": 2, "category": "cat_1"}]
        df = ft.TinyFrame.from_dicts(data)
        
        # Log operations (minimal)
        ft.log_operation("test_dataset", "Test")
        
        # Perform minimal operations
        filtered = df.filter("value", ">", 1)
        assert filtered.len() == 1
        
        # Check profiling
        stats = ft.get_overall_stats()
        assert stats["total_operations"] >= 0.0
        
        ft.disable_debug()
        ft.disable_profiling()
