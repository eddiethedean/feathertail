import pytest
import feathertail as ft


class TestWindowFunctions:
    """Test window functions (rolling and expanding)"""

    def get_column_data(self, frame, column_name):
        """Helper to get column data from a frame"""
        data = frame.to_dicts()
        return [row[column_name] for row in data]

    def test_rolling_mean_basic(self):
        """Test basic rolling mean calculation"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 4.0, "id": 4},
            {"value": 5.0, "id": 5},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 3)

        assert "value_rolling_mean" in result.columns
        assert result.len() == 5

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        # First two values should be None (insufficient data)
        assert rolling_means[0] is None
        assert rolling_means[1] is None
        # Third value: mean of [1, 2, 3] = 2.0
        assert rolling_means[2] == 2.0
        # Fourth value: mean of [2, 3, 4] = 3.0
        assert rolling_means[3] == 3.0
        # Fifth value: mean of [3, 4, 5] = 4.0
        assert rolling_means[4] == 4.0

    def test_rolling_mean_window_size_1(self):
        """Test rolling mean with window size 1"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 3.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 1)

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means == [1.0, 2.0, 3.0]

    def test_rolling_mean_window_size_larger_than_data(self):
        """Test rolling mean with window size larger than data"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 5)

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        # All values should be None (insufficient data)
        assert rolling_means == [None, None]

    def test_rolling_sum_basic(self):
        """Test basic rolling sum calculation"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 4.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_sum("value", 2)

        rolling_sums = self.get_column_data(result, "value_rolling_sum")
        # First value should be None (insufficient data)
        assert rolling_sums[0] is None
        # Second value: sum of [1, 2] = 3.0
        assert rolling_sums[1] == 3.0
        # Third value: sum of [2, 3] = 5.0
        assert rolling_sums[2] == 5.0
        # Fourth value: sum of [3, 4] = 7.0
        assert rolling_sums[3] == 7.0

    def test_rolling_std_basic(self):
        """Test basic rolling standard deviation calculation"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 4.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_std("value", 3)

        rolling_stds = self.get_column_data(result, "value_rolling_std")
        # First two values should be None (insufficient data)
        assert rolling_stds[0] is None
        assert rolling_stds[1] is None
        # Third value: std of [1, 2, 3] ≈ 0.816
        assert abs(rolling_stds[2] - 0.816) < 0.01
        # Fourth value: std of [2, 3, 4] ≈ 0.816
        assert abs(rolling_stds[3] - 0.816) < 0.01

    def test_rolling_with_integer_column(self):
        """Test rolling operations with integer column"""
        data = [
            {"value": 1, "id": 1},
            {"value": 2, "id": 2},
            {"value": 3, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 2)

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means[0] is None
        assert rolling_means[1] == 1.5  # (1 + 2) / 2
        assert rolling_means[2] == 2.5  # (2 + 3) / 2

    def test_rolling_with_optional_float_column(self):
        """Test rolling operations with optional float column"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": None, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 4.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 2)

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means[0] is None
        assert rolling_means[1] is None  # Only 1.0 available, need 2 for window size 2
        assert rolling_means[2] is None  # Window [None, 3.0] only has 1 non-null value
        assert rolling_means[3] == 3.5  # (3.0 + 4.0) / 2

    def test_expanding_mean_basic(self):
        """Test basic expanding mean calculation"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 4.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.expanding_mean("value")

        expanding_means = self.get_column_data(result, "value_expanding_mean")
        assert expanding_means[0] == 1.0  # mean of [1]
        assert expanding_means[1] == 1.5  # mean of [1, 2]
        assert expanding_means[2] == 2.0  # mean of [1, 2, 3]
        assert expanding_means[3] == 2.5  # mean of [1, 2, 3, 4]

    def test_expanding_sum_basic(self):
        """Test basic expanding sum calculation"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 3.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.expanding_sum("value")

        expanding_sums = self.get_column_data(result, "value_expanding_sum")
        assert expanding_sums[0] == 1.0  # sum of [1]
        assert expanding_sums[1] == 3.0  # sum of [1, 2]
        assert expanding_sums[2] == 6.0  # sum of [1, 2, 3]

    def test_expanding_with_integer_column(self):
        """Test expanding operations with integer column"""
        data = [
            {"value": 1, "id": 1},
            {"value": 2, "id": 2},
            {"value": 3, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.expanding_mean("value")

        expanding_means = self.get_column_data(result, "value_expanding_mean")
        assert expanding_means[0] == 1.0
        assert expanding_means[1] == 1.5
        assert expanding_means[2] == 2.0

    def test_expanding_with_optional_float_column(self):
        """Test expanding operations with optional float column"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": None, "id": 2},
            {"value": 3.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.expanding_mean("value")

        expanding_means = self.get_column_data(result, "value_expanding_mean")
        assert expanding_means[0] == 1.0  # mean of [1.0]
        assert expanding_means[1] == 1.0  # mean of [1.0] (None ignored)
        assert expanding_means[2] == 2.0  # mean of [1.0, 3.0]

    def test_window_operations_nonexistent_column(self):
        """Test window operations with non-existent column"""
        data = [{"value": 1.0}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(KeyError, match="Column 'nonexistent' not found"):
            frame.rolling_mean("nonexistent", 2)

    def test_window_operations_non_numeric_column(self):
        """Test window operations with non-numeric column"""
        data = [{"value": "string"}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(TypeError, match="Rolling operations only supported on numeric columns"):
            frame.rolling_mean("value", 2)

        with pytest.raises(TypeError, match="Expanding operations only supported on numeric columns"):
            frame.expanding_mean("value")

    def test_window_operations_empty_frame(self):
        """Test window operations with empty frame"""
        data = []
        frame = ft.TinyFrame.from_dicts([{"value": 1.0}]).filter("value", ">", 10)  # Create empty frame with schema

        result = frame.rolling_mean("value", 2)

        assert "value_rolling_mean" in result.columns
        assert result.len() == 0
        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means == []

    def test_window_operations_single_value(self):
        """Test window operations with single value"""
        data = [{"value": 5.0}]
        frame = ft.TinyFrame.from_dicts(data)

        # Rolling with window size 1 should work
        result = frame.rolling_mean("value", 1)
        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means == [5.0]

        # Rolling with window size > 1 should return None
        result = frame.rolling_mean("value", 2)
        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means == [None]

        # Expanding should work
        result = frame.expanding_mean("value")
        expanding_means = self.get_column_data(result, "value_expanding_mean")
        assert expanding_means == [5.0]

    def test_window_operations_large_dataset(self):
        """Test window operations with larger dataset"""
        data = []
        for i in range(100):
            data.append({"value": float(i), "id": i})

        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 10)

        assert "value_rolling_mean" in result.columns
        assert result.len() == 100

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        # First 9 values should be None
        for i in range(9):
            assert rolling_means[i] is None
        # 10th value should be mean of [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] = 4.5
        assert rolling_means[9] == 4.5

    def test_window_operations_chaining(self):
        """Test chaining window operations"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 4.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Chain rolling and expanding operations
        result = frame.rolling_mean("value", 2).expanding_mean("value")

        assert "value_rolling_mean" in result.columns
        assert "value_expanding_mean" in result.columns
        assert result.len() == 4

    def test_window_operations_with_negative_values(self):
        """Test window operations with negative values"""
        data = [
            {"value": -2.0, "id": 1},
            {"value": -1.0, "id": 2},
            {"value": 0.0, "id": 3},
            {"value": 1.0, "id": 4},
            {"value": 2.0, "id": 5},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 3)

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means[0] is None
        assert rolling_means[1] is None
        assert rolling_means[2] == -1.0  # mean of [-2, -1, 0]
        assert rolling_means[3] == 0.0   # mean of [-1, 0, 1]
        assert rolling_means[4] == 1.0   # mean of [0, 1, 2]

    def test_window_operations_with_zero_values(self):
        """Test window operations with zero values"""
        data = [
            {"value": 0.0, "id": 1},
            {"value": 0.0, "id": 2},
            {"value": 0.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_mean("value", 2)

        rolling_means = self.get_column_data(result, "value_rolling_mean")
        assert rolling_means[0] is None
        assert rolling_means[1] == 0.0  # mean of [0, 0]
        assert rolling_means[2] == 0.0  # mean of [0, 0]

    def test_window_operations_std_with_constant_values(self):
        """Test rolling std with constant values (should be 0)"""
        data = [
            {"value": 5.0, "id": 1},
            {"value": 5.0, "id": 2},
            {"value": 5.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rolling_std("value", 2)

        rolling_stds = self.get_column_data(result, "value_rolling_std")
        assert rolling_stds[0] is None
        assert rolling_stds[1] == 0.0  # std of [5, 5] = 0
        assert rolling_stds[2] == 0.0  # std of [5, 5] = 0
