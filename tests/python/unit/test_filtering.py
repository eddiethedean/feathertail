"""
Unit tests for filtering and sorting functionality.
"""

import pytest
import feathertail as ft
import pandas as pd
import numpy as np


class TestFiltering:
    """Test filtering operations."""

    @pytest.fixture
    def sample_data(self):
        """Sample data for filtering testing."""
        return [
            {"name": "Alice", "age": 25, "score": 85.5, "city": "New York"},
            {"name": "Bob", "age": 30, "score": 92.0, "city": "Paris"},
            {"name": "Charlie", "age": 35, "score": 78.5, "city": "London"},
            {"name": "Diana", "age": 28, "score": 88.0, "city": "New York"},
            {"name": "Eve", "age": 22, "score": 95.0, "city": "Tokyo"},
        ]

    @pytest.fixture
    def sample_frame(self, sample_data):
        """Sample TinyFrame for filtering testing."""
        return ft.TinyFrame.from_dicts(sample_data)

    def test_filter_equals(self, sample_frame):
        """Test filtering with equals condition."""
        result = sample_frame.filter("city", "==", "New York")
        
        assert result.len() == 2
        data = result.to_dicts()
        for row in data:
            assert row["city"] == "New York"

    def test_filter_not_equals(self, sample_frame):
        """Test filtering with not equals condition."""
        result = sample_frame.filter("city", "!=", "New York")
        
        assert result.len() == 3
        data = result.to_dicts()
        for row in data:
            assert row["city"] != "New York"

    def test_filter_greater_than(self, sample_frame):
        """Test filtering with greater than condition."""
        result = sample_frame.filter("age", ">", 28)
        
        assert result.len() == 2
        data = result.to_dicts()
        for row in data:
            assert row["age"] > 28

    def test_filter_less_than(self, sample_frame):
        """Test filtering with less than condition."""
        result = sample_frame.filter("age", "<", 30)
        
        assert result.len() == 3
        data = result.to_dicts()
        for row in data:
            assert row["age"] < 30

    def test_filter_greater_equal(self, sample_frame):
        """Test filtering with greater than or equal condition."""
        result = sample_frame.filter("age", ">=", 30)
        
        assert result.len() == 2
        data = result.to_dicts()
        for row in data:
            assert row["age"] >= 30

    def test_filter_less_equal(self, sample_frame):
        """Test filtering with less than or equal condition."""
        result = sample_frame.filter("age", "<=", 28)
        
        assert result.len() == 3
        data = result.to_dicts()
        for row in data:
            assert row["age"] <= 28

    def test_filter_nonexistent_column(self, sample_frame):
        """Test filtering with non-existent column should raise error."""
        with pytest.raises(Exception):  # Should raise KeyError
            sample_frame.filter("nonexistent", "==", "value")

    def test_filter_invalid_condition(self, sample_frame):
        """Test filtering with invalid condition should raise error."""
        with pytest.raises(Exception):  # Should raise ValueError
            sample_frame.filter("age", "invalid", 30)

    def test_dropna_no_nulls(self, sample_frame):
        """Test dropna with no null values."""
        result = sample_frame.dropna("age")
        
        # Should return the same frame since there are no nulls
        assert result.len() == sample_frame.len()
        assert result.shape == sample_frame.shape

    def test_dropna_with_nulls(self):
        """Test dropna with null values."""
        data_with_nulls = [
            {"name": "Alice", "age": 25, "score": 85.5},
            {"name": "Bob", "age": None, "score": 92.0},
            {"name": "Charlie", "age": 35, "score": None},
            {"name": "Diana", "age": 28, "score": 88.0},
        ]
        
        frame = ft.TinyFrame.from_dicts(data_with_nulls)
        
        # Drop nulls from age column
        result = frame.dropna("age")
        assert result.len() == 3  # Bob should be removed
        
        data = result.to_dicts()
        for row in data:
            assert row["age"] is not None

    def test_dropna_nonexistent_column(self, sample_frame):
        """Test dropna with non-existent column should raise error."""
        with pytest.raises(Exception):  # Should raise KeyError
            sample_frame.dropna("nonexistent")


class TestSorting:
    """Test sorting operations."""

    @pytest.fixture
    def sample_data(self):
        """Sample data for sorting testing."""
        return [
            {"name": "Alice", "age": 25, "score": 85.5},
            {"name": "Bob", "age": 30, "score": 92.0},
            {"name": "Charlie", "age": 35, "score": 78.5},
            {"name": "Diana", "age": 28, "score": 88.0},
            {"name": "Eve", "age": 22, "score": 95.0},
        ]

    @pytest.fixture
    def sample_frame(self, sample_data):
        """Sample TinyFrame for sorting testing."""
        return ft.TinyFrame.from_dicts(sample_data)

    def test_sort_single_column_ascending(self, sample_frame):
        """Test sorting by single column in ascending order."""
        result = sample_frame.sort_values(["age"], ascending=True)
        
        assert result.len() == sample_frame.len()
        assert result.shape == sample_frame.shape
        
        # Check that ages are in ascending order
        data = result.to_dicts()
        ages = [row["age"] for row in data]
        assert ages == sorted(ages)

    def test_sort_single_column_descending(self, sample_frame):
        """Test sorting by single column in descending order."""
        result = sample_frame.sort_values(["age"], ascending=False)
        
        assert result.len() == sample_frame.len()
        assert result.shape == sample_frame.shape
        
        # Check that ages are in descending order
        data = result.to_dicts()
        ages = [row["age"] for row in data]
        assert ages == sorted(ages, reverse=True)

    def test_sort_default_ascending(self, sample_frame):
        """Test sorting with default ascending order."""
        result = sample_frame.sort_values(["age"])
        
        # Should be ascending by default
        data = result.to_dicts()
        ages = [row["age"] for row in data]
        assert ages == sorted(ages)

    def test_sort_multiple_columns(self, sample_frame):
        """Test sorting by multiple columns."""
        # Add a second column with ties
        data_with_ties = [
            {"name": "Alice", "age": 25, "score": 85.5, "group": "A"},
            {"name": "Bob", "age": 25, "score": 92.0, "group": "B"},
            {"name": "Charlie", "age": 25, "score": 78.5, "group": "C"},
            {"name": "Diana", "age": 30, "score": 88.0, "group": "A"},
            {"name": "Eve", "age": 30, "score": 95.0, "group": "B"},
        ]
        
        frame = ft.TinyFrame.from_dicts(data_with_ties)
        result = frame.sort_values(["age", "score"], ascending=True)
        
        assert result.len() == frame.len()
        
        # Check that sorting is stable and correct
        data = result.to_dicts()
        for i in range(len(data) - 1):
            current = data[i]
            next_row = data[i + 1]
            
            if current["age"] == next_row["age"]:
                assert current["score"] <= next_row["score"]
            else:
                assert current["age"] < next_row["age"]

    def test_sort_nonexistent_column(self, sample_frame):
        """Test sorting with non-existent column should raise error."""
        with pytest.raises(Exception):  # Should raise KeyError
            sample_frame.sort_values(["nonexistent"])

    def test_sort_empty_frame(self):
        """Test sorting empty frame."""
        empty_frame = ft.TinyFrame()
        result = empty_frame.sort_values(["age"])
        
        assert result.len() == 0
        assert result.shape == (0, 0)

    def test_sort_single_row(self):
        """Test sorting single row frame."""
        single_row = [{"name": "Alice", "age": 25, "score": 85.5}]
        frame = ft.TinyFrame.from_dicts(single_row)
        result = frame.sort_values(["age"])
        
        assert result.len() == 1
        assert result.shape == frame.shape


class TestFilteringEdgeCases:
    """Test filtering edge cases and error conditions."""

    def test_filter_empty_result(self):
        """Test filtering that results in empty frame."""
        data = [{"name": "Alice", "age": 25}, {"name": "Bob", "age": 30}]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.filter("age", ">", 100)  # No ages > 100
        
        assert result.len() == 0
        assert result.shape == (0, 2)  # Same columns, no rows

    def test_filter_all_results(self):
        """Test filtering that returns all rows."""
        data = [{"name": "Alice", "age": 25}, {"name": "Bob", "age": 30}]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.filter("age", ">=", 0)  # All ages >= 0
        
        assert result.len() == frame.len()
        assert result.shape == frame.shape

    def test_sort_with_nulls(self):
        """Test sorting with null values."""
        data_with_nulls = [
            {"name": "Alice", "age": 25, "score": 85.5},
            {"name": "Bob", "age": None, "score": 92.0},
            {"name": "Charlie", "age": 35, "score": 78.5},
        ]
        
        frame = ft.TinyFrame.from_dicts(data_with_nulls)
        result = frame.sort_values(["age"])
        
        # Should handle nulls gracefully (nulls typically go to end)
        assert result.len() == frame.len()
        data = result.to_dicts()
        
        # Check that non-null ages are sorted
        non_null_ages = [row["age"] for row in data if row["age"] is not None]
        assert non_null_ages == sorted(non_null_ages)
