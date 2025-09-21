import pytest
import feathertail as ft
from feathertail import TinyFrame


class TestJoinOperations:
    """Test join operations on TinyFrame."""

    @pytest.fixture
    def left_frame(self):
        """Create a left frame for join tests."""
        data = [
            {"id": 1, "name": "Alice", "dept_id": 10},
            {"id": 2, "name": "Bob", "dept_id": 20},
            {"id": 3, "name": "Charlie", "dept_id": 10},
            {"id": 4, "name": "David", "dept_id": 30},
        ]
        return TinyFrame.from_dicts(data)

    @pytest.fixture
    def right_frame(self):
        """Create a right frame for join tests."""
        data = [
            {"dept_id": 10, "dept_name": "Engineering"},
            {"dept_id": 20, "dept_name": "Marketing"},
            {"dept_id": 40, "dept_name": "Sales"},
        ]
        return TinyFrame.from_dicts(data)

    def test_inner_join(self, left_frame, right_frame):
        """Test inner join operation."""
        result = left_frame.inner_join(right_frame, ["dept_id"], ["dept_id"])
        
        assert result.len() == 3
        assert "id" in result.columns
        assert "name" in result.columns
        assert "dept_id" in result.columns
        assert "dept_name" in result.columns
        
        # Check specific rows
        rows = list(result)
        dept_names = [row["dept_name"] for row in rows]
        assert "Engineering" in dept_names
        assert "Marketing" in dept_names
        assert "Sales" not in dept_names

    def test_left_join(self, left_frame, right_frame):
        """Test left join operation."""
        result = left_frame.left_join(right_frame, ["dept_id"], ["dept_id"])
        
        assert result.len() == 4  # All left rows preserved
        assert "id" in result.columns
        assert "name" in result.columns
        assert "dept_id" in result.columns
        assert "dept_name" in result.columns
        
        # Check that David (dept_id 30) has null dept_name
        rows = list(result)
        david_row = next(row for row in rows if row["name"] == "David")
        assert david_row["dept_name"] is None

    def test_right_join(self, left_frame, right_frame):
        """Test right join operation."""
        result = left_frame.right_join(right_frame, ["dept_id"], ["dept_id"])
        
        assert result.len() == 4  # All right rows preserved
        assert "id" in result.columns
        assert "name" in result.columns
        assert "dept_id" in result.columns
        assert "dept_name" in result.columns
        
        # Check that Sales (dept_id 40) has null id and name
        rows = list(result)
        sales_row = next(row for row in rows if row["dept_name"] == "Sales")
        assert sales_row["id"] is None
        assert sales_row["name"] is None

    def test_outer_join(self, left_frame, right_frame):
        """Test outer join operation."""
        result = left_frame.outer_join(right_frame, ["dept_id"], ["dept_id"])
        
        assert result.len() == 5  # All rows from both frames
        assert "id" in result.columns
        assert "name" in result.columns
        assert "dept_id" in result.columns
        assert "dept_name" in result.columns
        
        # Check that both David and Sales are present
        rows = list(result)
        names = [row["name"] for row in rows if row["name"] is not None]
        dept_names = [row["dept_name"] for row in rows if row["dept_name"] is not None]
        
        assert "David" in names
        assert "Sales" in dept_names

    def test_cross_join(self, left_frame, right_frame):
        """Test cross join operation."""
        result = left_frame.cross_join(right_frame)
        
        # Cross join should have 4 * 3 = 12 rows
        assert result.len() == 12
        assert "id" in result.columns
        assert "name" in result.columns
        assert "dept_id" in result.columns
        assert "dept_name" in result.columns

    def test_join_multiple_columns(self, left_frame):
        """Test join on multiple columns."""
        # Create a frame with multiple join columns
        data = [
            {"id": 1, "name": "Alice", "dept_id": 10, "location": "NYC"},
            {"id": 2, "name": "Bob", "dept_id": 20, "location": "SF"},
        ]
        left = TinyFrame.from_dicts(data)
        
        right_data = [
            {"dept_id": 10, "location": "NYC", "dept_name": "Engineering"},
            {"dept_id": 20, "location": "SF", "dept_name": "Marketing"},
        ]
        right = TinyFrame.from_dicts(right_data)
        
        result = left.inner_join(right, ["dept_id", "location"], ["dept_id", "location"])
        
        assert result.len() == 2
        assert "dept_name" in result.columns

    def test_join_nonexistent_column(self, left_frame, right_frame):
        """Test join with non-existent column raises error."""
        with pytest.raises(KeyError):
            left_frame.inner_join(right_frame, ["nonexistent"], ["dept_id"])

    def test_join_mismatched_columns(self, left_frame, right_frame):
        """Test join with mismatched number of columns raises error."""
        with pytest.raises(ValueError):
            left_frame.inner_join(right_frame, ["dept_id"], ["dept_id", "extra"])

    def test_join_empty_frames(self):
        """Test join with empty frames."""
        # Create empty frames with some columns
        empty1 = TinyFrame.from_dicts([{"id": None, "name": None}])
        empty1 = empty1.filter("id", "!=", None)  # This will result in empty frame
        
        empty2 = TinyFrame.from_dicts([{"id": None, "dept": None}])
        empty2 = empty2.filter("id", "!=", None)  # This will result in empty frame
        
        result = empty1.inner_join(empty2, ["id"], ["id"])
        assert result.len() == 0

    def test_join_different_types(self):
        """Test join with different data types."""
        left_data = [
            {"id": 1, "value": 100},
            {"id": 2, "value": 200},
        ]
        left = TinyFrame.from_dicts(left_data)
        
        right_data = [
            {"id": 1, "label": "A"},
            {"id": 2, "label": "B"},
        ]
        right = TinyFrame.from_dicts(right_data)
        
        result = left.inner_join(right, ["id"], ["id"])
        
        assert result.len() == 2
        assert "value" in result.columns
        assert "label" in result.columns

    def test_join_with_nulls(self):
        """Test join with null values in join columns."""
        left_data = [
            {"id": 1, "name": "Alice"},
            {"id": None, "name": "Bob"},
            {"id": 3, "name": "Charlie"},
        ]
        left = TinyFrame.from_dicts(left_data)
        
        right_data = [
            {"id": 1, "dept": "Engineering"},
            {"id": 2, "dept": "Marketing"},
            {"id": 3, "dept": "Sales"},
        ]
        right = TinyFrame.from_dicts(right_data)
        
        result = left.inner_join(right, ["id"], ["id"])
        
        # Should only include rows with non-null join values
        assert result.len() == 2
        names = [row["name"] for row in result]
        assert "Alice" in names
        assert "Charlie" in names
        assert "Bob" not in names

    def test_join_column_name_conflicts(self):
        """Test join with conflicting column names."""
        left_data = [{"id": 1, "name": "Alice", "value": 100}]
        left = TinyFrame.from_dicts(left_data)
        
        right_data = [{"id": 1, "dept_name": "Engineering", "dept_value": "Dept"}]
        right = TinyFrame.from_dicts(right_data)
        
        result = left.inner_join(right, ["id"], ["id"])
        
        # Should have all columns
        assert result.len() == 1
        assert "id" in result.columns
        assert "name" in result.columns
        assert "value" in result.columns
        assert "dept_name" in result.columns
        assert "dept_value" in result.columns


class TestJoinEdgeCases:
    """Test edge cases for join operations."""

    def test_join_single_row_frames(self):
        """Test join with single row frames."""
        left = TinyFrame.from_dicts([{"id": 1, "name": "Alice"}])
        right = TinyFrame.from_dicts([{"id": 1, "dept": "Engineering"}])
        
        result = left.inner_join(right, ["id"], ["id"])
        assert result.len() == 1
        rows = list(result)
        assert rows[0]["name"] == "Alice"
        assert rows[0]["dept"] == "Engineering"

    def test_join_no_matches(self):
        """Test join with no matching rows."""
        left = TinyFrame.from_dicts([{"id": 1, "name": "Alice"}])
        right = TinyFrame.from_dicts([{"id": 2, "dept": "Engineering"}])
        
        result = left.inner_join(right, ["id"], ["id"])
        assert result.len() == 0

    def test_join_duplicate_keys(self):
        """Test join with duplicate keys."""
        left_data = [
            {"id": 1, "name": "Alice"},
            {"id": 1, "name": "Alice2"},
        ]
        left = TinyFrame.from_dicts(left_data)
        
        right_data = [
            {"id": 1, "dept": "Engineering"},
            {"id": 1, "dept": "Engineering2"},
        ]
        right = TinyFrame.from_dicts(right_data)
        
        result = left.inner_join(right, ["id"], ["id"])
        
        # Should have 2 * 2 = 4 rows (cartesian product of duplicates)
        assert result.len() == 4

    def test_join_large_frames(self):
        """Test join with larger frames."""
        # Create frames with 100 rows each
        left_data = [{"id": i, "value": i * 10} for i in range(100)]
        left = TinyFrame.from_dicts(left_data)
        
        right_data = [{"id": i, "label": f"Item{i}"} for i in range(50, 150)]
        right = TinyFrame.from_dicts(right_data)
        
        result = left.inner_join(right, ["id"], ["id"])
        
        # Should have 50 matching rows (id 50-99)
        assert result.len() == 50
