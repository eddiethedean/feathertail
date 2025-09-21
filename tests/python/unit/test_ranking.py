import pytest
import feathertail as ft


class TestRankingFunctions:
    """Test ranking functions (rank and pct_change)"""

    def get_column_data(self, frame, column_name):
        """Helper to get column data from a frame"""
        data = frame.to_dicts()
        return [row[column_name] for row in data]

    def test_rank_average_basic(self):
        """Test basic ranking with average method"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 4.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        assert "value_rank" in result.columns
        assert result.len() == 4

        ranks = self.get_column_data(result, "value_rank")
        # Sorted values: [1.0, 2.0, 3.0, 4.0]
        # Ranks: [1.0, 2.0, 3.0, 4.0]
        assert ranks == [3.0, 1.0, 4.0, 2.0]

    def test_rank_with_ties_average(self):
        """Test ranking with ties using average method"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        ranks = self.get_column_data(result, "value_rank")
        # Sorted values: [1.0, 2.0, 3.0, 3.0]
        # Ranks: [1.0, 2.0, 3.5, 3.5] (average of 3 and 4)
        assert ranks == [3.5, 1.0, 3.5, 2.0]

    def test_rank_min_method(self):
        """Test ranking with min method for ties"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "min")

        ranks = self.get_column_data(result, "value_rank")
        # Sorted values: [1.0, 2.0, 3.0, 3.0]
        # Ranks: [1.0, 2.0, 3.0, 3.0] (min rank for ties)
        assert ranks == [3.0, 1.0, 3.0, 2.0]

    def test_rank_max_method(self):
        """Test ranking with max method for ties"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "max")

        ranks = self.get_column_data(result, "value_rank")
        # Sorted values: [1.0, 2.0, 3.0, 3.0]
        # Ranks: [1.0, 2.0, 4.0, 4.0] (max rank for ties)
        assert ranks == [4.0, 1.0, 4.0, 2.0]

    def test_rank_first_method(self):
        """Test ranking with first method (no special handling for ties)"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "first")

        ranks = self.get_column_data(result, "value_rank")
        # Sorted values: [1.0, 2.0, 3.0, 3.0]
        # Ranks: [1.0, 2.0, 3.0, 4.0] (first occurrence gets lower rank)
        assert ranks == [3.0, 1.0, 4.0, 2.0]

    def test_rank_dense_method(self):
        """Test ranking with dense method (no gaps)"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 3.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "dense")

        ranks = self.get_column_data(result, "value_rank")
        # Sorted values: [1.0, 2.0, 3.0, 3.0]
        # Dense ranks: [1.0, 2.0, 3.0, 3.0] (no gaps)
        assert ranks == [3.0, 1.0, 3.0, 2.0]

    def test_rank_with_integer_column(self):
        """Test ranking with integer column"""
        data = [
            {"value": 3, "id": 1},
            {"value": 1, "id": 2},
            {"value": 4, "id": 3},
            {"value": 2, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        ranks = self.get_column_data(result, "value_rank")
        assert ranks == [3.0, 1.0, 4.0, 2.0]

    def test_rank_with_optional_float_column(self):
        """Test ranking with optional float column"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": None, "id": 2},
            {"value": 1.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        ranks = self.get_column_data(result, "value_rank")
        # Only non-null values are ranked: [1.0, 2.0, 3.0]
        # Ranks: [3.0, None, 1.0, 2.0]
        assert ranks == [3.0, None, 1.0, 2.0]

    def test_rank_invalid_method(self):
        """Test ranking with invalid method"""
        data = [{"value": 1.0}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(ValueError, match="Invalid ranking method"):
            frame.rank("value", "invalid")

    def test_rank_nonexistent_column(self):
        """Test ranking with non-existent column"""
        data = [{"value": 1.0}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(KeyError, match="Column 'nonexistent' not found"):
            frame.rank("nonexistent", "average")

    def test_rank_non_numeric_column(self):
        """Test ranking with non-numeric column"""
        data = [{"value": "string"}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(TypeError, match="Ranking only supported on numeric columns"):
            frame.rank("value", "average")

    def test_rank_empty_frame(self):
        """Test ranking with empty frame"""
        data = []
        frame = ft.TinyFrame.from_dicts([{"value": 1.0}]).filter("value", ">", 10)  # Create empty frame with schema

        result = frame.rank("value", "average")

        assert "value_rank" in result.columns
        assert result.len() == 0
        ranks = self.get_column_data(result, "value_rank")
        assert ranks == []

    def test_rank_single_value(self):
        """Test ranking with single value"""
        data = [{"value": 5.0}]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        ranks = self.get_column_data(result, "value_rank")
        assert ranks == [1.0]

    def test_rank_all_same_values(self):
        """Test ranking with all same values"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 1.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        ranks = self.get_column_data(result, "value_rank")
        # All values are tied, so they all get the average rank
        assert ranks == [2.0, 2.0, 2.0]  # (1 + 2 + 3) / 3 = 2.0

    def test_pct_change_basic(self):
        """Test basic percentage change calculation"""
        data = [
            {"value": 100.0, "id": 1},
            {"value": 110.0, "id": 2},
            {"value": 121.0, "id": 3},
            {"value": 100.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.pct_change("value")

        assert "value_pct_change" in result.columns
        assert result.len() == 4

        pct_changes = self.get_column_data(result, "value_pct_change")
        assert pct_changes[0] is None  # First value is always None
        assert abs(pct_changes[1] - 10.0) < 0.001  # (110-100)/100 * 100 = 10%
        assert abs(pct_changes[2] - 10.0) < 0.001  # (121-110)/110 * 100 = 10%
        assert abs(pct_changes[3] - (-17.36)) < 0.1  # (100-121)/121 * 100 ≈ -17.36%

    def test_pct_change_with_integer_column(self):
        """Test percentage change with integer column"""
        data = [
            {"value": 100, "id": 1},
            {"value": 120, "id": 2},
            {"value": 90, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.pct_change("value")

        pct_changes = self.get_column_data(result, "value_pct_change")
        assert pct_changes[0] is None
        assert abs(pct_changes[1] - 20.0) < 0.001  # (120-100)/100 * 100 = 20%
        assert abs(pct_changes[2] - (-25.0)) < 0.001  # (90-120)/120 * 100 = -25%

    def test_pct_change_with_optional_float_column(self):
        """Test percentage change with optional float column"""
        data = [
            {"value": 100.0, "id": 1},
            {"value": None, "id": 2},
            {"value": 120.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.pct_change("value")

        pct_changes = self.get_column_data(result, "value_pct_change")
        assert pct_changes[0] is None
        assert pct_changes[1] is None  # Previous value is None
        assert pct_changes[2] is None  # Previous value is None (None -> 120.0)

    def test_pct_change_with_zero_values(self):
        """Test percentage change with zero values"""
        data = [
            {"value": 0.0, "id": 1},
            {"value": 10.0, "id": 2},
            {"value": 0.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.pct_change("value")

        pct_changes = self.get_column_data(result, "value_pct_change")
        assert pct_changes[0] is None
        assert pct_changes[1] is None  # Division by zero (0.0 -> 10.0)
        assert abs(pct_changes[2] - (-100.0)) < 0.001  # (0.0 - 10.0) / 10.0 * 100 = -100%

    def test_pct_change_nonexistent_column(self):
        """Test percentage change with non-existent column"""
        data = [{"value": 1.0}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(KeyError, match="Column 'nonexistent' not found"):
            frame.pct_change("nonexistent")

    def test_pct_change_non_numeric_column(self):
        """Test percentage change with non-numeric column"""
        data = [{"value": "string"}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(TypeError, match="Percentage change only supported on numeric columns"):
            frame.pct_change("value")

    def test_pct_change_empty_frame(self):
        """Test percentage change with empty frame"""
        data = []
        frame = ft.TinyFrame.from_dicts([{"value": 1.0}]).filter("value", ">", 10)  # Create empty frame with schema

        result = frame.pct_change("value")

        assert "value_pct_change" in result.columns
        assert result.len() == 0
        pct_changes = self.get_column_data(result, "value_pct_change")
        assert pct_changes == []

    def test_pct_change_single_value(self):
        """Test percentage change with single value"""
        data = [{"value": 5.0}]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.pct_change("value")

        pct_changes = self.get_column_data(result, "value_pct_change")
        assert pct_changes == [None]  # First value is always None

    def test_ranking_chaining(self):
        """Test chaining ranking operations"""
        data = [
            {"value": 3.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 4.0, "id": 3},
            {"value": 2.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Chain ranking and percentage change
        result = frame.rank("value", "average").pct_change("value")

        assert "value_rank" in result.columns
        assert "value_pct_change" in result.columns
        assert result.len() == 4

    def test_ranking_large_dataset(self):
        """Test ranking with larger dataset"""
        data = []
        for i in range(100):
            data.append({"value": float(i % 10), "id": i})

        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        assert "value_rank" in result.columns
        assert result.len() == 100

        ranks = self.get_column_data(result, "value_rank")
        # All values should have ranks
        assert all(rank is not None for rank in ranks)
        # Ranks should be between 1 and 100 (since we have 100 values with 10 unique values)
        assert all(1.0 <= rank <= 100.0 for rank in ranks if rank is not None)

    def test_ranking_with_negative_values(self):
        """Test ranking with negative values"""
        data = [
            {"value": -2.0, "id": 1},
            {"value": -1.0, "id": 2},
            {"value": 0.0, "id": 3},
            {"value": 1.0, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "average")

        ranks = self.get_column_data(result, "value_rank")
        assert ranks == [1.0, 2.0, 3.0, 4.0]

    def test_ranking_with_duplicates_dense(self):
        """Test dense ranking with many duplicates"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 1.0, "id": 2},
            {"value": 1.0, "id": 3},
            {"value": 2.0, "id": 4},
            {"value": 2.0, "id": 5},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.rank("value", "dense")

        ranks = self.get_column_data(result, "value_rank")
        # Dense ranking: [1.0, 1.0, 1.0, 2.0, 2.0]
        assert ranks == [1.0, 1.0, 1.0, 2.0, 2.0]
