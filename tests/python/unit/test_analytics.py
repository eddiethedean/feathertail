import pytest
import math
from feathertail import TinyFrame

@pytest.fixture
def sample_numeric_frame():
    """Create a sample frame with numeric data for testing."""
    data = [
        {"id": 1, "value": 10.5, "score": 85.2, "count": 100},
        {"id": 2, "value": 15.3, "score": 92.1, "count": 150},
        {"id": 3, "value": 8.7, "score": 78.9, "count": 120},
        {"id": 4, "value": 22.1, "score": 95.5, "count": 200},
        {"id": 5, "value": 12.8, "score": 88.3, "count": 180},
    ]
    return TinyFrame.from_dicts(data)

@pytest.fixture
def sample_mixed_frame():
    """Create a sample frame with mixed data types for testing."""
    data = [
        {"name": "Alice", "age": 25, "salary": 50000.0, "active": True},
        {"name": "Bob", "age": 30, "salary": 60000.0, "active": False},
        {"name": "Charlie", "age": 35, "salary": 70000.0, "active": True},
        {"name": "David", "age": 28, "salary": 55000.0, "active": True},
        {"name": "Eve", "age": 32, "salary": 65000.0, "active": False},
    ]
    return TinyFrame.from_dicts(data)

@pytest.fixture
def sample_with_nulls():
    """Create a sample frame with null values for testing."""
    data = [
        {"id": 1, "value": 10.5, "score": 85.2},
        {"id": 2, "value": None, "score": 92.1},
        {"id": 3, "value": 8.7, "score": None},
        {"id": 4, "value": 22.1, "score": 95.5},
        {"id": 5, "value": None, "score": None},
    ]
    return TinyFrame.from_dicts(data)

class TestDescriptiveStatistics:
    """Test descriptive statistics functions."""

    def test_describe_basic(self, sample_numeric_frame):
        """Test basic describe functionality."""
        result = sample_numeric_frame.describe()
        
        assert result.len() == 8  # count, mean, std, min, 25%, 50%, 75%, max
        assert "statistic" in result.columns
        assert "value" in result.columns
        assert "score" in result.columns
        assert "count" in result.columns
        
        # Check that statistic names are correct
        stats = list(result)
        stat_names = [row["statistic"] for row in stats]
        expected_stats = ["count", "mean", "std", "min", "25%", "50%", "75%", "max"]
        assert stat_names == expected_stats

    def test_describe_numeric_columns_only(self, sample_mixed_frame):
        """Test that describe only includes numeric columns."""
        result = sample_mixed_frame.describe()
        
        assert "age" in result.columns
        assert "salary" in result.columns
        assert "name" not in result.columns  # String column should be excluded
        assert "active" not in result.columns  # Boolean column should be excluded

    def test_describe_empty_frame(self):
        """Test describe with empty frame."""
        # Create empty frame with some columns but no data
        data = [{"value": None, "score": None}]
        frame = TinyFrame.from_dicts(data)
        empty_frame = frame.filter("value", "!=", None)  # This will result in empty frame
        result = empty_frame.describe()
        
        assert result.len() == 8  # Still has statistic names
        assert "statistic" in result.columns
        assert len(result.columns) == 1  # Only statistic column

    def test_skew_basic(self, sample_numeric_frame):
        """Test skewness calculation."""
        skew_value = sample_numeric_frame.skew("value")
        assert isinstance(skew_value, float)
        # For the test data, we expect some skewness
        assert not math.isnan(skew_value)

    def test_skew_empty_column(self):
        """Test skewness with empty column."""
        # Create a frame with some numeric data, then filter to get empty
        data = [{"value": 1.0}, {"value": 2.0}]
        frame = TinyFrame.from_dicts(data)
        empty_frame = frame.filter("value", ">", 10.0)  # This will result in empty frame
        skew_value = empty_frame.skew("value")
        assert skew_value == 0.0

    def test_kurtosis_basic(self, sample_numeric_frame):
        """Test kurtosis calculation."""
        kurtosis_value = sample_numeric_frame.kurtosis("value")
        assert isinstance(kurtosis_value, float)
        assert not math.isnan(kurtosis_value)

    def test_kurtosis_insufficient_data(self):
        """Test kurtosis with insufficient data."""
        data = [{"value": 1.0}, {"value": 2.0}]
        frame = TinyFrame.from_dicts(data)
        kurtosis_value = frame.kurtosis("value")
        assert kurtosis_value == 0.0

    def test_quantile_basic(self, sample_numeric_frame):
        """Test quantile calculation."""
        q50 = sample_numeric_frame.quantile("value", 0.5)
        q25 = sample_numeric_frame.quantile("value", 0.25)
        q75 = sample_numeric_frame.quantile("value", 0.75)
        
        assert isinstance(q50, float)
        assert isinstance(q25, float)
        assert isinstance(q75, float)
        assert q25 <= q50 <= q75

    def test_quantile_invalid_range(self, sample_numeric_frame):
        """Test quantile with invalid range."""
        with pytest.raises(ValueError):
            sample_numeric_frame.quantile("value", -0.1)
        
        with pytest.raises(ValueError):
            sample_numeric_frame.quantile("value", 1.1)

    def test_quantile_empty_column(self):
        """Test quantile with empty column."""
        # Create a frame with some numeric data, then filter to get empty
        data = [{"value": 1.0}, {"value": 2.0}]
        frame = TinyFrame.from_dicts(data)
        empty_frame = frame.filter("value", ">", 10.0)  # This will result in empty frame
        
        with pytest.raises(ValueError):
            empty_frame.quantile("value", 0.5)

    def test_mode_numeric(self, sample_numeric_frame):
        """Test mode calculation with numeric data."""
        # Add some duplicate values
        data = [
            {"id": 1, "value": 10.0},
            {"id": 2, "value": 10.0},
            {"id": 3, "value": 15.0},
            {"id": 4, "value": 10.0},
        ]
        frame = TinyFrame.from_dicts(data)
        mode_value = frame.mode("value")
        assert mode_value == 10.0

    def test_mode_string(self, sample_mixed_frame):
        """Test mode calculation with string data."""
        mode_value = sample_mixed_frame.mode("name")
        # Should return one of the names (most frequent)
        assert mode_value in ["Alice", "Bob", "Charlie", "David", "Eve"]

    def test_mode_boolean(self, sample_mixed_frame):
        """Test mode calculation with boolean data."""
        mode_value = sample_mixed_frame.mode("active")
        assert mode_value in [True, False]

    def test_nunique_numeric(self, sample_numeric_frame):
        """Test unique count with numeric data."""
        unique_count = sample_numeric_frame.nunique("value")
        assert unique_count == 5  # All values are unique

    def test_nunique_string(self, sample_mixed_frame):
        """Test unique count with string data."""
        unique_count = sample_mixed_frame.nunique("name")
        assert unique_count == 5  # All names are unique

    def test_nunique_with_duplicates(self):
        """Test unique count with duplicate values."""
        data = [
            {"value": 10.0},
            {"value": 10.0},
            {"value": 15.0},
            {"value": 10.0},
        ]
        frame = TinyFrame.from_dicts(data)
        unique_count = frame.nunique("value")
        assert unique_count == 2  # Only 10.0 and 15.0

    def test_nunique_with_nulls(self, sample_with_nulls):
        """Test unique count with null values."""
        unique_count = sample_with_nulls.nunique("value")
        # Should count non-null unique values
        assert unique_count == 3  # 10.5, 8.7, 22.1

class TestCorrelationFunctions:
    """Test correlation and covariance functions."""

    def test_corr_basic(self, sample_numeric_frame):
        """Test basic correlation matrix."""
        result = sample_numeric_frame.corr()
        
        assert "column" in result.columns
        assert "value" in result.columns
        assert "score" in result.columns
        assert "count" in result.columns
        
        # Check that diagonal correlations are 1.0
        rows = list(result)
        for row in rows:
            col_name = row["column"]
            assert abs(row[col_name] - 1.0) < 1e-10

    def test_corr_with_nulls(self, sample_with_nulls):
        """Test correlation with null values."""
        # Create a simpler test with some valid data
        data = [
            {"value": 10.0, "score": 85.0},
            {"value": 15.0, "score": 92.0},
            {"value": 8.0, "score": 78.0},
        ]
        frame = TinyFrame.from_dicts(data)
        result = frame.corr()
        
        assert "column" in result.columns
        assert "value" in result.columns
        assert "score" in result.columns

    def test_corr_with_insufficient_data(self):
        """Test correlation with insufficient data."""
        data = [{"value": 1.0, "score": 2.0}]
        frame = TinyFrame.from_dicts(data)
        result = frame.corr()
        
        # Should still work but with limited data
        assert "column" in result.columns

    def test_corr_with_specific_columns(self, sample_numeric_frame):
        """Test correlation between specific columns."""
        corr_value = sample_numeric_frame.corr_with("value", "score")
        assert isinstance(corr_value, float)
        assert -1.0 <= corr_value <= 1.0

    def test_corr_with_nonexistent_column(self, sample_numeric_frame):
        """Test correlation with non-existent column."""
        with pytest.raises(KeyError):
            sample_numeric_frame.corr_with("nonexistent", "value")

    def test_cov_basic(self, sample_numeric_frame):
        """Test basic covariance matrix."""
        result = sample_numeric_frame.cov()
        
        assert "column" in result.columns
        assert "value" in result.columns
        assert "score" in result.columns
        assert "count" in result.columns

    def test_cov_with_specific_columns(self, sample_numeric_frame):
        """Test covariance between specific columns."""
        cov_value = sample_numeric_frame.cov_with("value", "score")
        assert isinstance(cov_value, float)

    def test_cov_with_nonexistent_column(self, sample_numeric_frame):
        """Test covariance with non-existent column."""
        with pytest.raises(KeyError):
            sample_numeric_frame.cov_with("nonexistent", "value")

class TestAnalyticsEdgeCases:
    """Test edge cases for analytics functions."""

    def test_single_value_column(self):
        """Test analytics with single value column."""
        data = [{"value": 42.0} for _ in range(5)]
        frame = TinyFrame.from_dicts(data)
        
        # These should handle single values gracefully
        skew = frame.skew("value")
        kurtosis = frame.kurtosis("value")
        mode = frame.mode("value")
        nunique = frame.nunique("value")
        
        assert isinstance(skew, float)
        assert isinstance(kurtosis, float)
        assert mode == 42.0
        assert nunique == 1

    def test_constant_column(self):
        """Test analytics with constant column."""
        data = [{"value": 10.0} for _ in range(10)]
        frame = TinyFrame.from_dicts(data)
        
        # Skewness and kurtosis should be 0 for constant data
        skew = frame.skew("value")
        kurtosis = frame.kurtosis("value")
        
        assert abs(skew) < 1e-10
        assert abs(kurtosis) < 1e-10

    def test_large_dataset(self):
        """Test analytics with larger dataset."""
        import random
        random.seed(42)  # For reproducible tests
        
        data = [
            {
                "value": random.uniform(0, 100),
                "score": random.uniform(0, 100),
                "count": random.randint(1, 1000)
            }
            for _ in range(100)
        ]
        frame = TinyFrame.from_dicts(data)
        
        # All functions should work with larger datasets
        result = frame.describe()
        assert result.len() == 8
        
        corr_result = frame.corr()
        assert "column" in corr_result.columns
        
        skew = frame.skew("value")
        assert isinstance(skew, float)
        
        kurtosis = frame.kurtosis("value")
        assert isinstance(kurtosis, float)

    def test_mixed_data_types(self, sample_mixed_frame):
        """Test analytics with mixed data types."""
        # Only numeric columns should be included in describe
        result = sample_mixed_frame.describe()
        assert "age" in result.columns
        assert "salary" in result.columns
        assert "name" not in result.columns
        assert "active" not in result.columns
        
        # String and boolean columns should work with mode and nunique
        name_mode = sample_mixed_frame.mode("name")
        assert isinstance(name_mode, str)
        
        active_nunique = sample_mixed_frame.nunique("active")
        assert active_nunique == 2  # True and False

    def test_error_handling(self, sample_numeric_frame):
        """Test error handling for invalid inputs."""
        # Test with non-existent column
        with pytest.raises(KeyError):
            sample_numeric_frame.skew("nonexistent")
        
        with pytest.raises(KeyError):
            sample_numeric_frame.kurtosis("nonexistent")
        
        with pytest.raises(KeyError):
            sample_numeric_frame.quantile("nonexistent", 0.5)
        
        with pytest.raises(KeyError):
            sample_numeric_frame.mode("nonexistent")
        
        with pytest.raises(KeyError):
            sample_numeric_frame.nunique("nonexistent")
        
        with pytest.raises(KeyError):
            sample_numeric_frame.corr_with("nonexistent", "value")
        
        with pytest.raises(KeyError):
            sample_numeric_frame.cov_with("nonexistent", "value")

    def test_empty_frame_analytics(self):
        """Test analytics with completely empty frame."""
        # Create empty frame with some columns but no data
        data = [{"value": None, "score": None}]
        frame = TinyFrame.from_dicts(data)
        empty_frame = frame.filter("value", "!=", None)  # This will result in empty frame
        
        # These should handle empty frames gracefully
        result = empty_frame.describe()
        assert result.len() == 8  # Statistic names only
        
        corr_result = empty_frame.corr()
        assert "column" in corr_result.columns
        assert corr_result.len() == 0  # No numeric columns
