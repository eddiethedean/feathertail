import pytest
import feathertail as ft

class TestValidationOperations:
    """Test validation operations"""

    def get_column_data(self, frame, column_name):
        """Helper to get column data from a frame"""
        data = frame.to_dicts()
        return [row[column_name] for row in data]

    def test_validate_not_null_basic(self):
        """Test basic not null validation"""
        data = [
            {"value": 1, "id": 1},
            {"value": 2, "id": 2},
            {"value": 3, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_not_null("value")

        assert "value_not_null" in result.columns
        assert result.len() == 3

        validation_results = self.get_column_data(result, "value_not_null")
        assert validation_results == [True, True, True]

    def test_validate_not_null_with_nulls(self):
        """Test not null validation with null values"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": None, "id": 2},
            {"value": 3.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_not_null("value")

        validation_results = self.get_column_data(result, "value_not_null")
        assert validation_results == [True, False, True]

    def test_validate_range_basic(self):
        """Test basic range validation"""
        data = [
            {"value": 5, "id": 1},
            {"value": 10, "id": 2},
            {"value": 15, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_range("value", 0, 20)

        assert "value_in_range" in result.columns
        assert result.len() == 3

        validation_results = self.get_column_data(result, "value_in_range")
        assert validation_results == [True, True, True]

    def test_validate_range_with_violations(self):
        """Test range validation with violations"""
        data = [
            {"value": 5, "id": 1},
            {"value": 25, "id": 2},  # Out of range
            {"value": 15, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_range("value", 0, 20)

        validation_results = self.get_column_data(result, "value_in_range")
        assert validation_results == [True, False, True]

    def test_validate_range_with_nulls(self):
        """Test range validation with null values"""
        data = [
            {"value": 5.0, "id": 1},
            {"value": None, "id": 2},  # Null should be valid
            {"value": 25.0, "id": 3},  # Out of range
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_range("value", 0.0, 20.0)

        validation_results = self.get_column_data(result, "value_in_range")
        assert validation_results == [True, True, False]

    def test_validate_range_min_only(self):
        """Test range validation with only minimum value"""
        data = [
            {"value": 5, "id": 1},
            {"value": 2, "id": 2},  # Below minimum
            {"value": 15, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_range("value", 5, None)

        validation_results = self.get_column_data(result, "value_in_range")
        assert validation_results == [True, False, True]

    def test_validate_range_max_only(self):
        """Test range validation with only maximum value"""
        data = [
            {"value": 5, "id": 1},
            {"value": 25, "id": 2},  # Above maximum
            {"value": 15, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_range("value", None, 20)

        validation_results = self.get_column_data(result, "value_in_range")
        assert validation_results == [True, False, True]

    def test_validate_pattern_basic(self):
        """Test basic pattern validation"""
        data = [
            {"email": "user@example.com", "id": 1},
            {"email": "admin@test.org", "id": 2},
            {"email": "invalid-email", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_pattern("email", "@")

        assert "email_matches_pattern" in result.columns
        assert result.len() == 3

        validation_results = self.get_column_data(result, "email_matches_pattern")
        assert validation_results == [True, True, False]

    def test_validate_pattern_with_nulls(self):
        """Test pattern validation with null values"""
        data = [
            {"email": "user@example.com", "id": 1},
            {"email": None, "id": 2},  # Null should be valid
            {"email": "invalid-email", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_pattern("email", "@")

        validation_results = self.get_column_data(result, "email_matches_pattern")
        assert validation_results == [True, True, False]

    def test_validate_unique_basic(self):
        """Test basic uniqueness validation"""
        data = [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"},
            {"id": 3, "name": "Alice"},  # Duplicate
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_unique("name")

        assert "name_unique" in result.columns
        assert result.len() == 3

        validation_results = self.get_column_data(result, "name_unique")
        assert validation_results == [True, True, False]

    def test_validate_unique_with_nulls(self):
        """Test uniqueness validation with null values"""
        data = [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": None},  # Null should be valid
            {"id": 3, "name": "Alice"},  # Duplicate
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_unique("name")

        validation_results = self.get_column_data(result, "name_unique")
        assert validation_results == [True, True, False]

    def test_validate_unique_numeric(self):
        """Test uniqueness validation with numeric values"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": 2.0, "id": 2},
            {"value": 1.0, "id": 3},  # Duplicate
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_unique("value")

        validation_results = self.get_column_data(result, "value_unique")
        assert validation_results == [True, True, False]

    def test_validation_summary_basic(self):
        """Test basic validation summary"""
        data = [
            {"value": 1, "id": 1},
            {"value": 2, "id": 2},
            {"value": 3, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        summary = frame.validation_summary("value")

        assert summary["total_count"] == 3.0
        assert summary["null_count"] == 0.0
        assert summary["null_percentage"] == 0.0
        assert summary["non_null_count"] == 3.0

    def test_validation_summary_with_nulls(self):
        """Test validation summary with null values"""
        data = [
            {"value": 1.0, "id": 1},
            {"value": None, "id": 2},
            {"value": 3.0, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        summary = frame.validation_summary("value")

        assert summary["total_count"] == 3.0
        assert summary["null_count"] == 1.0
        assert abs(summary["null_percentage"] - 33.333333333333336) < 0.001
        assert summary["non_null_count"] == 2.0

    def test_validation_operations_nonexistent_column(self):
        """Test validation operations with non-existent column"""
        data = [{"value": 1}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(KeyError, match="Column 'nonexistent' not found"):
            frame.validate_not_null("nonexistent")

    def test_validation_operations_wrong_type(self):
        """Test validation operations with wrong column type"""
        data = [{"value": "string"}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(TypeError, match="Range validation only supported for numeric columns"):
            frame.validate_range("value", 0, 10)

        data2 = [{"value": 123}]
        frame2 = ft.TinyFrame.from_dicts(data2)
        with pytest.raises(TypeError, match="Pattern validation only supported for string columns"):
            frame2.validate_pattern("value", "pattern")

    def test_validation_operations_empty_frame(self):
        """Test validation operations with empty frame"""
        data = []
        frame = ft.TinyFrame.from_dicts([{"value": 1, "id": 1}]).filter("id", ">", 10)  # Create empty frame with schema

        result = frame.validate_not_null("value")

        assert "value_not_null" in result.columns
        assert result.len() == 0
        validation_results = self.get_column_data(result, "value_not_null")
        assert validation_results == []

    def test_validation_operations_chaining(self):
        """Test chaining multiple validation operations"""
        data = [
            {"value": 5, "name": "Alice"},
            {"value": 25, "name": "Bob"},  # Out of range
            {"value": 15, "name": "Alice"},  # Duplicate name
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_range("value", 0, 20).validate_unique("name")

        assert "value_in_range" in result.columns
        assert "name_unique" in result.columns
        assert result.len() == 3

        range_results = self.get_column_data(result, "value_in_range")
        unique_results = self.get_column_data(result, "name_unique")
        assert range_results == [True, False, True]
        assert unique_results == [True, True, False]

    def test_validation_operations_large_dataset(self):
        """Test validation operations with larger dataset"""
        data = []
        for i in range(100):
            data.append({"value": float(i), "id": i})

        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_range("value", 0.0, 100.0)

        assert "value_in_range" in result.columns
        assert result.len() == 100

        validation_results = self.get_column_data(result, "value_in_range")
        assert all(validation_results)  # All should be valid

    def test_validation_operations_edge_cases(self):
        """Test validation operations with edge cases"""
        data = [
            {"value": 0, "id": 1},
            {"value": 10, "id": 2},
            {"value": 20, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test exact boundary values
        result = frame.validate_range("value", 0, 20)
        validation_results = self.get_column_data(result, "value_in_range")
        assert validation_results == [True, True, True]

        # Test with boundary values excluded
        result = frame.validate_range("value", 1, 19)
        validation_results = self.get_column_data(result, "value_in_range")
        assert validation_results == [False, True, False]

    def test_validation_operations_boolean_values(self):
        """Test validation operations with boolean values"""
        data = [
            {"flag": True, "id": 1},
            {"flag": False, "id": 2},
            {"flag": True, "id": 3},  # Duplicate
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_unique("flag")

        validation_results = self.get_column_data(result, "flag_unique")
        assert validation_results == [True, True, False]

    def test_validation_operations_string_patterns(self):
        """Test validation operations with various string patterns"""
        data = [
            {"code": "ABC123", "id": 1},
            {"code": "XYZ789", "id": 2},
            {"code": "ABC456", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test pattern matching
        result = frame.validate_pattern("code", "ABC")
        pattern_results = self.get_column_data(result, "code_matches_pattern")
        assert pattern_results == [True, False, True]

        # Test uniqueness
        result = frame.validate_unique("code")
        unique_results = self.get_column_data(result, "code_unique")
        assert unique_results == [True, True, True]  # All unique

    def test_validation_operations_mixed_types(self):
        """Test validation operations with mixed data types"""
        data = [
            {"value": 1, "text": "hello", "id": 1},
            {"value": None, "text": "world", "id": 2},
            {"value": 3, "text": None, "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test not null validation on different types
        result1 = frame.validate_not_null("value")
        result2 = frame.validate_not_null("text")

        value_results = self.get_column_data(result1, "value_not_null")
        text_results = self.get_column_data(result2, "text_not_null")
        assert value_results == [True, False, True]
        assert text_results == [True, True, False]

    def test_validation_operations_performance(self):
        """Test validation operations performance with large dataset"""
        data = []
        for i in range(1000):
            data.append({"value": float(i % 100), "id": i})  # Some duplicates

        frame = ft.TinyFrame.from_dicts(data)

        result = frame.validate_unique("value")

        assert "value_unique" in result.columns
        assert result.len() == 1000

        validation_results = self.get_column_data(result, "value_unique")
        # First occurrence of each value should be True, subsequent should be False
        assert validation_results[0] == True  # First occurrence of 0.0
        assert validation_results[100] == False  # Second occurrence of 0.0
