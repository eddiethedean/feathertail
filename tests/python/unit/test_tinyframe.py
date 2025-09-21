"""
Unit tests for TinyFrame functionality.
"""

import pytest
import feathertail as ft
import pandas as pd
import numpy as np


class TestTinyFrameCreation:
    """Test TinyFrame creation and basic properties."""

    def test_empty_frame_creation(self):
        """Test creating an empty TinyFrame."""
        frame = ft.TinyFrame()
        assert frame.len() == 0
        assert frame.is_empty()
        assert frame.shape == (0, 0)

    def test_frame_creation_from_dicts(self, sample_records):
        """Test creating TinyFrame from list of dictionaries."""
        frame = ft.TinyFrame.from_dicts(sample_records)
        assert frame.len() == 5
        assert not frame.is_empty()
        assert frame.shape == (5, 4)

    def test_frame_creation_with_mixed_types(self, mixed_type_records):
        """Test creating TinyFrame with mixed data types."""
        frame = ft.TinyFrame.from_dicts(mixed_type_records)
        assert frame.len() == 4
        assert frame.shape == (4, 2)

    def test_frame_creation_empty_list(self):
        """Test creating TinyFrame from empty list should raise error."""
        with pytest.raises(Exception):  # Should raise ValueError
            ft.TinyFrame.from_dicts([])

    def test_frame_creation_single_record(self, single_record):
        """Test creating TinyFrame from single record."""
        frame = ft.TinyFrame.from_dicts(single_record)
        assert frame.len() == 1
        assert frame.shape == (1, 2)


class TestTinyFrameProperties:
    """Test TinyFrame properties and basic operations."""

    def test_len_property(self, sample_frame):
        """Test length property."""
        assert sample_frame.len() == 5

    def test_is_empty_property(self, sample_frame):
        """Test is_empty property."""
        assert not sample_frame.is_empty()
        
        empty_frame = ft.TinyFrame()
        assert empty_frame.is_empty()

    def test_shape_property(self, sample_frame):
        """Test shape property."""
        assert sample_frame.shape == (5, 4)

    def test_repr_method(self, sample_frame):
        """Test string representation."""
        repr_str = repr(sample_frame)
        assert "TinyFrame" in repr_str
        assert "rows=5" in repr_str
        assert "columns=4" in repr_str

    def test_iteration(self, sample_frame):
        """Test iteration over rows."""
        rows = list(sample_frame)
        assert len(rows) == 5
        assert all(isinstance(row, dict) for row in rows)
        assert "name" in rows[0]
        assert "age" in rows[0]


class TestTinyFrameOperations:
    """Test TinyFrame operations."""

    def test_fillna_scalar(self, sample_frame):
        """Test filling missing values with scalar."""
        # First, ensure we have some None values
        original_data = sample_frame.to_dicts()
        
        # Fill missing values
        sample_frame.fillna({"age": 0, "score": 0.0})
        
        # Check that None values were filled
        filled_data = sample_frame.to_dicts()
        for row in filled_data:
            assert row["age"] is not None
            assert row["score"] is not None

    def test_fillna_dict(self, sample_frame):
        """Test filling missing values with dictionary."""
        sample_frame.fillna({"age": 25, "score": 80.0})
        
        filled_data = sample_frame.to_dicts()
        for row in filled_data:
            if row["age"] is None:
                assert row["age"] == 25
            if row["score"] is None:
                assert row["score"] == 80.0

    def test_cast_column(self, sample_frame):
        """Test casting column types."""
        # Cast age to float
        sample_frame.cast_column("age", float)
        
        # Check that age column is now float
        data = sample_frame.to_dicts()
        for row in data:
            if row["age"] is not None:
                assert isinstance(row["age"], float)

    def test_edit_column(self, sample_frame):
        """Test editing column values."""
        # Edit name column to uppercase
        sample_frame.edit_column("name", lambda x: x.upper() if x else x)
        
        data = sample_frame.to_dicts()
        for row in data:
            if row["name"] is not None:
                assert row["name"].isupper()

    def test_drop_columns(self, sample_frame):
        """Test dropping columns."""
        original_shape = sample_frame.shape
        sample_frame.drop_columns(["score"])
        
        new_shape = sample_frame.shape
        assert new_shape[0] == original_shape[0]  # Same number of rows
        assert new_shape[1] == original_shape[1] - 1  # One less column

    def test_rename_column(self, sample_frame):
        """Test renaming columns."""
        sample_frame.rename_column("name", "full_name")
        
        data = sample_frame.to_dicts()
        for row in data:
            assert "full_name" in row
            assert "name" not in row


class TestTinyFrameEdgeCases:
    """Test edge cases and error conditions."""

    def test_drop_nonexistent_column(self, sample_frame):
        """Test dropping non-existent column should raise error."""
        with pytest.raises(Exception):
            sample_frame.drop_columns(["nonexistent"])

    def test_rename_nonexistent_column(self, sample_frame):
        """Test renaming non-existent column should raise error."""
        with pytest.raises(Exception):
            sample_frame.rename_column("nonexistent", "new_name")

    def test_rename_to_existing_column(self, sample_frame):
        """Test renaming to existing column name should raise error."""
        with pytest.raises(Exception):
            sample_frame.rename_column("name", "age")

    def test_cast_nonexistent_column(self, sample_frame):
        """Test casting non-existent column should raise error."""
        with pytest.raises(Exception):
            sample_frame.cast_column("nonexistent", int)

    def test_edit_nonexistent_column(self, sample_frame):
        """Test editing non-existent column should raise error."""
        with pytest.raises(Exception):
            sample_frame.edit_column("nonexistent", lambda x: x)


class TestTinyFrameComparison:
    """Test comparison with pandas DataFrame."""

    def test_basic_operations_comparison(self, sample_records, sample_pandas_frame):
        """Test that basic operations work similarly to pandas."""
        # Create feathertail frame
        ft_frame = ft.TinyFrame.from_dicts(sample_records)
        
        # Compare basic properties
        assert ft_frame.shape == sample_pandas_frame.shape
        assert ft_frame.len() == len(sample_pandas_frame)
        
        # Compare data
        ft_data = ft_frame.to_dicts()
        pd_data = sample_pandas_frame.to_dict("records")
        
        # Check that we have the same number of records
        assert len(ft_data) == len(pd_data)
        
        # Check that all keys are present
        ft_keys = set(ft_data[0].keys()) if ft_data else set()
        pd_keys = set(pd_data[0].keys()) if pd_data else set()
        assert ft_keys == pd_keys
