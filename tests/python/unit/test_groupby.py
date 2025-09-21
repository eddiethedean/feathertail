"""
Unit tests for TinyGroupBy functionality.
"""

import pytest
import feathertail as ft
import pandas as pd
import numpy as np


class TestTinyGroupBy:
    """Test TinyGroupBy operations."""

    @pytest.fixture
    def sample_data(self):
        """Sample data for GroupBy testing."""
        return [
            {"category": "A", "value": 10, "score": 85.5},
            {"category": "B", "value": 20, "score": 92.0},
            {"category": "A", "value": 15, "score": 78.5},
            {"category": "B", "value": 25, "score": 88.0},
            {"category": "A", "value": 12, "score": 91.5},
            {"category": "C", "value": 30, "score": 95.0},
        ]

    @pytest.fixture
    def sample_frame(self, sample_data):
        """Sample TinyFrame for GroupBy testing."""
        return ft.TinyFrame.from_dicts(sample_data)

    def test_groupby_creation(self, sample_frame):
        """Test creating a GroupBy object."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        assert groupby.keys == ["category"]
        assert len(groupby.groups) == 3  # A, B, C

    def test_groupby_count(self, sample_frame):
        """Test count aggregation."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        result = groupby.count(sample_frame)
        
        assert result.len() == 3
        assert result.shape == (3, 2)  # category + count columns
        
        # Check that we have the right categories
        data = result.to_dicts()
        categories = [row["category"] for row in data]
        assert "A" in categories
        assert "B" in categories
        assert "C" in categories

    def test_groupby_sum(self, sample_frame):
        """Test sum aggregation."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        result = groupby.sum(sample_frame, "value")
        
        assert result.len() == 3
        assert result.shape == (3, 2)  # category + value_sum columns
        
        # Check sum values
        data = result.to_dicts()
        for row in data:
            if row["category"] == "A":
                assert row["value_sum"] == 37  # 10 + 15 + 12
            elif row["category"] == "B":
                assert row["value_sum"] == 45  # 20 + 25
            elif row["category"] == "C":
                assert row["value_sum"] == 30

    def test_groupby_mean(self, sample_frame):
        """Test mean aggregation."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        result = groupby.mean(sample_frame, "score")
        
        assert result.len() == 3
        assert result.shape == (3, 2)  # category + score_mean columns
        
        # Check mean values
        data = result.to_dicts()
        for row in data:
            if row["category"] == "A":
                # (85.5 + 78.5 + 91.5) / 3 = 85.16666666666667
                assert abs(row["score_mean"] - 85.16666666666667) < 1e-6
            elif row["category"] == "B":
                assert abs(row["score_mean"] - 90.0) < 1e-6  # (92.0 + 88.0) / 2
            elif row["category"] == "C":
                assert row["score_mean"] == 95.0

    def test_groupby_min_max(self, sample_frame):
        """Test min and max aggregations."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        
        min_result = groupby.min(sample_frame, "value")
        max_result = groupby.max(sample_frame, "value")
        
        assert min_result.len() == 3
        assert max_result.len() == 3
        
        # Check min values
        min_data = min_result.to_dicts()
        for row in min_data:
            if row["category"] == "A":
                assert row["value_min"] == 10
            elif row["category"] == "B":
                assert row["value_min"] == 20
            elif row["category"] == "C":
                assert row["value_min"] == 30
        
        # Check max values
        max_data = max_result.to_dicts()
        for row in max_data:
            if row["category"] == "A":
                assert row["value_max"] == 15
            elif row["category"] == "B":
                assert row["value_max"] == 25
            elif row["category"] == "C":
                assert row["value_max"] == 30

    def test_groupby_std_var(self, sample_frame):
        """Test standard deviation and variance aggregations."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        
        std_result = groupby.std(sample_frame, "score")
        var_result = groupby.var(sample_frame, "score")
        
        assert std_result.len() == 3
        assert var_result.len() == 3
        
        # Check that std^2 ≈ var
        std_data = std_result.to_dicts()
        var_data = var_result.to_dicts()
        
        for i, (std_row, var_row) in enumerate(zip(std_data, var_data)):
            if std_row["score_std"] is not None and var_row["score_var"] is not None:
                assert abs(std_row["score_std"]**2 - var_row["score_var"]) < 1e-10

    def test_groupby_median(self, sample_frame):
        """Test median aggregation."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        result = groupby.median(sample_frame, "value")
        
        assert result.len() == 3
        
        # Check median values
        data = result.to_dicts()
        for row in data:
            if row["category"] == "A":
                assert row["value_median"] == 12  # median of [10, 15, 12] sorted = [10, 12, 15]
            elif row["category"] == "B":
                assert row["value_median"] == 22.5  # median of [20, 25] = (20 + 25) / 2
            elif row["category"] == "C":
                assert row["value_median"] == 30

    def test_groupby_first_last(self, sample_frame):
        """Test first and last aggregations."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        
        first_result = groupby.first(sample_frame, "value")
        last_result = groupby.last(sample_frame, "value")
        
        assert first_result.len() == 3
        assert last_result.len() == 3
        
        # Check first values (order preserved from original data)
        first_data = first_result.to_dicts()
        for row in first_data:
            if row["category"] == "A":
                assert row["value_first"] == 10  # first A value
            elif row["category"] == "B":
                assert row["value_first"] == 20  # first B value
            elif row["category"] == "C":
                assert row["value_first"] == 30  # first C value

    def test_groupby_size(self, sample_frame):
        """Test size aggregation (same as count)."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        result = groupby.size(sample_frame)
        
        assert result.len() == 3
        assert result.shape == (3, 2)  # category + size columns
        
        # Check size values
        data = result.to_dicts()
        for row in data:
            if row["category"] == "A":
                assert row["size"] == 3
            elif row["category"] == "B":
                assert row["size"] == 2
            elif row["category"] == "C":
                assert row["size"] == 1

    def test_groupby_multiple_keys(self):
        """Test GroupBy with multiple keys."""
        # Create data with multiple grouping columns
        extended_data = [
            {"category": "A", "subcategory": "X", "value": 10, "score": 85.5},
            {"category": "A", "subcategory": "Y", "value": 15, "score": 78.5},
            {"category": "B", "subcategory": "X", "value": 20, "score": 92.0},
            {"category": "B", "subcategory": "Y", "value": 25, "score": 88.0},
            {"category": "A", "subcategory": "X", "value": 12, "score": 91.5},
            {"category": "C", "subcategory": "Z", "value": 30, "score": 95.0},
        ]
        
        frame = ft.TinyFrame.from_dicts(extended_data)
        groupby = ft.TinyGroupBy(frame, ["category", "subcategory"])
        
        assert groupby.keys == ["category", "subcategory"]
        assert len(groupby.groups) > 3  # More groups due to subcategory
        
        result = groupby.count(frame)
        assert result.len() > 3

    def test_groupby_nonexistent_column(self, sample_frame):
        """Test GroupBy with non-existent column should raise error."""
        with pytest.raises(Exception):  # Should raise KeyError
            ft.TinyGroupBy(sample_frame, ["nonexistent"])

    def test_groupby_aggregation_nonexistent_column(self, sample_frame):
        """Test aggregation with non-existent column should raise error."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        
        with pytest.raises(Exception):  # Should raise KeyError
            groupby.sum(sample_frame, "nonexistent")

    def test_groupby_empty_frame(self):
        """Test GroupBy with empty frame."""
        empty_frame = ft.TinyFrame()
        
        with pytest.raises(Exception):  # Should raise error
            ft.TinyGroupBy(empty_frame, ["category"])

    def test_groupby_groups_property(self, sample_frame):
        """Test accessing groups property."""
        groupby = ft.TinyGroupBy(sample_frame, ["category"])
        groups = groupby.groups
        
        assert isinstance(groups, dict)
        assert len(groups) == 3
        
        # Check that groups contain the right row indices
        for key, indices in groups.items():
            assert isinstance(key, tuple)
            assert isinstance(indices, list)
            assert all(isinstance(i, int) for i in indices)
