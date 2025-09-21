import pytest
import feathertail as ft

class TestStringOperations:
    """Test string operations"""

    def get_column_data(self, frame, column_name):
        """Helper to get column data from a frame"""
        data = frame.to_dicts()
        return [row[column_name] for row in data]

    def test_str_upper_basic(self):
        """Test basic string uppercase conversion"""
        data = [
            {"text": "hello", "id": 1},
            {"text": "world", "id": 2},
            {"text": "python", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_upper("text")

        assert "text_upper" in result.columns
        assert result.len() == 3

        upper_texts = self.get_column_data(result, "text_upper")
        assert upper_texts == ["HELLO", "WORLD", "PYTHON"]

    def test_str_upper_with_optional_strings(self):
        """Test uppercase with optional string column"""
        data = [
            {"text": "hello", "id": 1},
            {"text": None, "id": 2},
            {"text": "world", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_upper("text")

        upper_texts = self.get_column_data(result, "text_upper")
        assert upper_texts == ["HELLO", None, "WORLD"]

    def test_str_lower_basic(self):
        """Test basic string lowercase conversion"""
        data = [
            {"text": "HELLO", "id": 1},
            {"text": "WORLD", "id": 2},
            {"text": "PYTHON", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_lower("text")

        assert "text_lower" in result.columns
        assert result.len() == 3

        lower_texts = self.get_column_data(result, "text_lower")
        assert lower_texts == ["hello", "world", "python"]

    def test_str_strip_basic(self):
        """Test basic string stripping"""
        data = [
            {"text": "  hello  ", "id": 1},
            {"text": "\tworld\n", "id": 2},
            {"text": "  python  ", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_strip("text")

        assert "text_strip" in result.columns
        assert result.len() == 3

        stripped_texts = self.get_column_data(result, "text_strip")
        assert stripped_texts == ["hello", "world", "python"]

    def test_str_replace_basic(self):
        """Test basic string replacement"""
        data = [
            {"text": "hello world", "id": 1},
            {"text": "world hello", "id": 2},
            {"text": "hello python", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_replace("text", "hello", "hi")

        assert "text_replace" in result.columns
        assert result.len() == 3

        replaced_texts = self.get_column_data(result, "text_replace")
        assert replaced_texts == ["hi world", "world hi", "hi python"]

    def test_str_split_basic(self):
        """Test basic string splitting"""
        data = [
            {"text": "a,b,c", "id": 1},
            {"text": "x,y,z", "id": 2},
            {"text": "1,2,3", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_split("text", ",")

        assert "text_split" in result.columns
        assert result.len() == 3

        split_texts = self.get_column_data(result, "text_split")
        assert split_texts == ["a|b|c", "x|y|z", "1|2|3"]

    def test_str_contains_basic(self):
        """Test basic string contains check"""
        data = [
            {"text": "hello world", "id": 1},
            {"text": "python programming", "id": 2},
            {"text": "data science", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_contains("text", "hello")

        assert "text_contains" in result.columns
        assert result.len() == 3

        contains_results = self.get_column_data(result, "text_contains")
        assert contains_results == [True, False, False]

    def test_str_len_basic(self):
        """Test basic string length calculation"""
        data = [
            {"text": "hello", "id": 1},
            {"text": "world", "id": 2},
            {"text": "python", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_len("text")

        assert "text_len" in result.columns
        assert result.len() == 3

        lengths = self.get_column_data(result, "text_len")
        assert lengths == [5, 5, 6]

    def test_str_cat_basic(self):
        """Test basic string concatenation"""
        data = [
            {"text": "hello", "id": 1},
            {"text": "world", "id": 2},
            {"text": "python", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_cat("text", " ")

        assert "text_cat" in result.columns
        assert result.len() == 3

        concatenated = self.get_column_data(result, "text_cat")
        assert concatenated == ["hello world python", "hello world python", "hello world python"]

    def test_str_operations_with_empty_strings(self):
        """Test string operations with empty strings"""
        data = [
            {"text": "", "id": 1},
            {"text": "hello", "id": 2},
            {"text": "", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test uppercase
        result = frame.str_upper("text")
        upper_texts = self.get_column_data(result, "text_upper")
        assert upper_texts == ["", "HELLO", ""]

        # Test length
        result = frame.str_len("text")
        lengths = self.get_column_data(result, "text_len")
        assert lengths == [0, 5, 0]

        # Test contains
        result = frame.str_contains("text", "hello")
        contains_results = self.get_column_data(result, "text_contains")
        assert contains_results == [False, True, False]

    def test_str_operations_with_special_characters(self):
        """Test string operations with special characters"""
        data = [
            {"text": "héllo wörld", "id": 1},
            {"text": "café", "id": 2},
            {"text": "naïve", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test uppercase
        result = frame.str_upper("text")
        upper_texts = self.get_column_data(result, "text_upper")
        assert upper_texts == ["HÉLLO WÖRLD", "CAFÉ", "NAÏVE"]

        # Test replace
        result = frame.str_replace("text", "é", "e")
        replaced_texts = self.get_column_data(result, "text_replace")
        assert replaced_texts == ["hello wörld", "cafe", "naïve"]

    def test_str_operations_nonexistent_column(self):
        """Test string operations with non-existent column"""
        data = [{"text": "hello"}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(KeyError, match="Column 'nonexistent' not found"):
            frame.str_upper("nonexistent")

    def test_str_operations_non_string_column(self):
        """Test string operations with non-string column"""
        data = [{"value": 123}]
        frame = ft.TinyFrame.from_dicts(data)

        with pytest.raises(TypeError, match="String operations only supported on string columns"):
            frame.str_upper("value")

    def test_str_operations_empty_frame(self):
        """Test string operations with empty frame"""
        data = []
        frame = ft.TinyFrame.from_dicts([{"text": "hello", "id": 1}]).filter("id", ">", 10)  # Create empty frame with schema

        result = frame.str_upper("text")

        assert "text_upper" in result.columns
        assert result.len() == 0
        upper_texts = self.get_column_data(result, "text_upper")
        assert upper_texts == []

    def test_str_operations_chaining(self):
        """Test chaining multiple string operations"""
        data = [
            {"text": "  Hello World  ", "id": 1},
            {"text": "  Python Programming  ", "id": 2},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_strip("text").str_lower("text_strip").str_replace("text_strip_lower", " ", "_")

        assert "text_strip" in result.columns
        assert "text_strip_lower" in result.columns
        assert "text_strip_lower_replace" in result.columns
        assert result.len() == 2

        final_texts = self.get_column_data(result, "text_strip_lower_replace")
        assert final_texts == ["hello_world", "python_programming"]

    def test_str_operations_large_dataset(self):
        """Test string operations with larger dataset"""
        data = []
        for i in range(100):
            data.append({"text": f"item_{i}", "id": i})

        frame = ft.TinyFrame.from_dicts(data)

        result = frame.str_upper("text")

        assert "text_upper" in result.columns
        assert result.len() == 100

        upper_texts = self.get_column_data(result, "text_upper")
        assert all(text.startswith("ITEM_") for text in upper_texts)
        assert all(isinstance(text, str) for text in upper_texts)

    def test_str_operations_with_nulls(self):
        """Test string operations with null values"""
        data = [
            {"text": "hello", "id": 1},
            {"text": None, "id": 2},
            {"text": "world", "id": 3},
            {"text": None, "id": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test uppercase
        result = frame.str_upper("text")
        upper_texts = self.get_column_data(result, "text_upper")
        assert upper_texts == ["HELLO", None, "WORLD", None]

        # Test length
        result = frame.str_len("text")
        lengths = self.get_column_data(result, "text_len")
        assert lengths == [5, None, 5, None]

        # Test contains
        result = frame.str_contains("text", "hello")
        contains_results = self.get_column_data(result, "text_contains")
        assert contains_results == [True, None, False, None]

    def test_str_operations_unicode(self):
        """Test string operations with unicode characters"""
        data = [
            {"text": "🚀 rocket", "id": 1},
            {"text": "🌟 star", "id": 2},
            {"text": "🎉 party", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test uppercase
        result = frame.str_upper("text")
        upper_texts = self.get_column_data(result, "text_upper")
        assert upper_texts == ["🚀 ROCKET", "🌟 STAR", "🎉 PARTY"]

        # Test length (unicode characters count as 1)
        result = frame.str_len("text")
        lengths = self.get_column_data(result, "text_len")
        assert lengths == [11, 9, 10]  # emoji (4 bytes) + space (1 byte) + word (6 bytes)

        # Test replace
        result = frame.str_replace("text", "🚀", "🚁")
        replaced_texts = self.get_column_data(result, "text_replace")
        assert replaced_texts == ["🚁 rocket", "🌟 star", "🎉 party"]

    def test_str_operations_edge_cases(self):
        """Test string operations with edge cases"""
        data = [
            {"text": "a", "id": 1},
            {"text": "ab", "id": 2},
            {"text": "abc", "id": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)

        # Test split with single character
        result = frame.str_split("text", "b")
        split_texts = self.get_column_data(result, "text_split")
        assert split_texts == ["a", "a|", "a|c"]

        # Test replace with empty string
        result = frame.str_replace("text", "b", "")
        replaced_texts = self.get_column_data(result, "text_replace")
        assert replaced_texts == ["a", "a", "ac"]

        # Test contains with empty substring
        result = frame.str_contains("text", "")
        contains_results = self.get_column_data(result, "text_contains")
        assert contains_results == [True, True, True]  # Empty string is always contained
