"""
Test time series operations for TinyFrame
"""

import pytest
import feathertail as ft
from datetime import datetime


class TestTimeSeriesOperations:
    """Test time series operations"""
    
    def get_column_data(self, frame, column_name):
        """Helper to get column data from a frame"""
        data = frame.to_dicts()
        return [row[column_name] for row in data]

    def test_to_timestamps_basic(self):
        """Test basic timestamp conversion"""
        data = [
            {"datetime": "2023-01-01 10:30:00", "value": 1},
            {"datetime": "2023-01-02 14:45:30", "value": 2},
            {"datetime": "2023-01-03 09:15:45", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.to_timestamps("datetime")
        
        assert "datetime_timestamp" in result.columns
        assert result.len() == 3
        
        # Check that timestamps are reasonable (around 2023)
        timestamps = self.get_column_data(result, "datetime_timestamp")
        assert all(ts > 1600000000 and ts < 2000000000 for ts in timestamps)

    def test_to_timestamps_different_formats(self):
        """Test timestamp conversion with different datetime formats"""
        data = [
            {"datetime": "2023-01-01", "value": 1},
            {"datetime": "2023-01-01T10:30:00", "value": 2},
            {"datetime": "2023-01-01T10:30:00.123", "value": 3},
            {"datetime": "2023-01-01T10:30:00.123Z", "value": 4},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.to_timestamps("datetime")
        
        assert "datetime_timestamp" in result.columns
        assert result.len() == 4

    def test_to_timestamps_empty_strings(self):
        """Test timestamp conversion with empty strings"""
        data = [
            {"datetime": "", "value": 1},
            {"datetime": "2023-01-01 10:30:00", "value": 2},
            {"datetime": "", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.to_timestamps("datetime")
        
        assert "datetime_timestamp" in result.columns
        assert result.len() == 3
        
        # Empty strings should result in 0 timestamps
        timestamps = self.get_column_data(result, "datetime_timestamp")
        assert timestamps[0] == 0
        assert timestamps[2] == 0
        assert timestamps[1] > 0

    def test_dt_year(self):
        """Test year extraction"""
        data = [
            {"datetime": "2023-01-01 10:30:00", "value": 1},
            {"datetime": "2024-06-15 14:45:30", "value": 2},
            {"datetime": "2022-12-31 23:59:59", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_year("datetime")
        
        assert "datetime_year" in result.columns
        assert result.len() == 3
        
        years = self.get_column_data(result, "datetime_year")
        assert years == [2023, 2024, 2022]

    def test_dt_month(self):
        """Test month extraction"""
        data = [
            {"datetime": "2023-01-01 10:30:00", "value": 1},
            {"datetime": "2023-06-15 14:45:30", "value": 2},
            {"datetime": "2023-12-31 23:59:59", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_month("datetime")
        
        assert "datetime_month" in result.columns
        assert result.len() == 3
        
        months = self.get_column_data(result, "datetime_month")
        assert months == [1, 6, 12]

    def test_dt_day(self):
        """Test day extraction"""
        data = [
            {"datetime": "2023-01-01 10:30:00", "value": 1},
            {"datetime": "2023-01-15 14:45:30", "value": 2},
            {"datetime": "2023-01-31 23:59:59", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_day("datetime")
        
        assert "datetime_day" in result.columns
        assert result.len() == 3
        
        days = self.get_column_data(result, "datetime_day")
        assert days == [1, 15, 31]

    def test_dt_hour(self):
        """Test hour extraction"""
        data = [
            {"datetime": "2023-01-01 00:30:00", "value": 1},
            {"datetime": "2023-01-01 12:45:30", "value": 2},
            {"datetime": "2023-01-01 23:59:59", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_hour("datetime")
        
        assert "datetime_hour" in result.columns
        assert result.len() == 3
        
        hours = self.get_column_data(result, "datetime_hour")
        assert hours == [0, 12, 23]

    def test_dt_minute(self):
        """Test minute extraction"""
        data = [
            {"datetime": "2023-01-01 10:00:00", "value": 1},
            {"datetime": "2023-01-01 10:30:00", "value": 2},
            {"datetime": "2023-01-01 10:59:00", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_minute("datetime")
        
        assert "datetime_minute" in result.columns
        assert result.len() == 3
        
        minutes = self.get_column_data(result, "datetime_minute")
        assert minutes == [0, 30, 59]

    def test_dt_second(self):
        """Test second extraction"""
        data = [
            {"datetime": "2023-01-01 10:30:00", "value": 1},
            {"datetime": "2023-01-01 10:30:30", "value": 2},
            {"datetime": "2023-01-01 10:30:59", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_second("datetime")
        
        assert "datetime_second" in result.columns
        assert result.len() == 3
        
        seconds = self.get_column_data(result, "datetime_second")
        assert seconds == [0, 30, 59]

    def test_dt_day_of_week(self):
        """Test day of week extraction"""
        data = [
            {"datetime": "2023-01-01 10:30:00", "value": 1},  # Sunday
            {"datetime": "2023-01-02 10:30:00", "value": 2},  # Monday
            {"datetime": "2023-01-07 10:30:00", "value": 3},  # Saturday
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_day_of_week("datetime")
        
        assert "datetime_day_of_week" in result.columns
        assert result.len() == 3
        
        days_of_week = self.get_column_data(result, "datetime_day_of_week")
        assert days_of_week == [0, 1, 6]  # Sunday=0, Monday=1, Saturday=6

    def test_dt_day_of_year(self):
        """Test day of year extraction"""
        data = [
            {"datetime": "2023-01-01 10:30:00", "value": 1},  # Day 1
            {"datetime": "2023-06-15 10:30:00", "value": 2},  # Day 166
            {"datetime": "2023-12-31 10:30:00", "value": 3},  # Day 365
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_day_of_year("datetime")
        
        assert "datetime_day_of_year" in result.columns
        assert result.len() == 3
        
        days_of_year = self.get_column_data(result, "datetime_day_of_year")
        assert days_of_year == [1, 166, 365]

    def test_dt_diff(self):
        """Test time difference calculation"""
        data = [
            {"datetime": "2023-01-01 10:00:00", "value": 1},
            {"datetime": "2023-01-01 10:30:00", "value": 2},  # +30 minutes
            {"datetime": "2023-01-01 11:00:00", "value": 3},  # +30 minutes
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_diff("datetime")
        
        assert "datetime_diff" in result.columns
        assert result.len() == 3
        
        diffs = self.get_column_data(result, "datetime_diff")
        assert diffs[0] is None  # First element has no previous
        assert diffs[1] == 1800  # 30 minutes = 1800 seconds
        assert diffs[2] == 1800  # 30 minutes = 1800 seconds

    def test_dt_shift(self):
        """Test timestamp shifting"""
        data = [
            {"datetime": "2023-01-01 10:00:00", "value": 1},
            {"datetime": "2023-01-01 10:30:00", "value": 2},
            {"datetime": "2023-01-01 11:00:00", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_shift("datetime", 3600)  # Shift by 1 hour
        
        assert "datetime_shifted" in result.columns
        assert result.len() == 3
        
        shifted = self.get_column_data(result, "datetime_shifted")
        # Check that times are shifted by 1 hour
        assert "2023-01-01 11:00:00" in shifted[0]
        assert "2023-01-01 11:30:00" in shifted[1]
        assert "2023-01-01 12:00:00" in shifted[2]

    def test_dt_shift_negative(self):
        """Test negative timestamp shifting"""
        data = [
            {"datetime": "2023-01-01 10:00:00", "value": 1},
            {"datetime": "2023-01-01 10:30:00", "value": 2},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_shift("datetime", -1800)  # Shift back by 30 minutes
        
        assert "datetime_shifted" in result.columns
        assert result.len() == 2
        
        shifted = self.get_column_data(result, "datetime_shifted")
        assert "2023-01-01 09:30:00" in shifted[0]
        assert "2023-01-01 10:00:00" in shifted[1]

    def test_time_series_with_existing_timestamps(self):
        """Test time series operations with existing timestamp column"""
        data = [
            {"timestamp": 1672574400, "value": 1},  # 2023-01-01 10:00:00
            {"timestamp": 1672576200, "value": 2},  # 2023-01-01 10:30:00
            {"timestamp": 1672578000, "value": 3},  # 2023-01-01 11:00:00
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_year("timestamp")
        
        assert "timestamp_year" in result.columns
        assert result.len() == 3
        
        years = self.get_column_data(result, "timestamp_year")
        assert years == [2023, 2023, 2023]

    def test_time_series_with_optional_strings(self):
        """Test time series operations with optional string column"""
        data = [
            {"datetime": "2023-01-01 10:00:00", "value": 1},
            {"datetime": None, "value": 2},
            {"datetime": "2023-01-01 11:00:00", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_year("datetime")
        
        assert "datetime_year" in result.columns
        assert result.len() == 3
        
        years = self.get_column_data(result, "datetime_year")
        assert years[0] == 2023
        assert years[1] == 0  # None should result in 0
        assert years[2] == 2023

    def test_time_series_nonexistent_column(self):
        """Test time series operations with non-existent column"""
        data = [{"value": 1}, {"value": 2}]
        frame = ft.TinyFrame.from_dicts(data)
        
        with pytest.raises(KeyError):
            frame.dt_year("nonexistent")

    def test_time_series_wrong_column_type(self):
        """Test time series operations with wrong column type"""
        data = [{"value": 1.5}, {"value": 2.5}]
        frame = ft.TinyFrame.from_dicts(data)
        
        with pytest.raises(TypeError):
            frame.dt_year("value")

    def test_time_series_empty_frame(self):
        """Test time series operations with empty frame"""
        data = []
        
        with pytest.raises(ValueError):
            frame = ft.TinyFrame.from_dicts(data)

    def test_time_series_chaining(self):
        """Test chaining multiple time series operations"""
        data = [
            {"datetime": "2023-01-01 10:00:00", "value": 1},
            {"datetime": "2023-01-01 10:30:00", "value": 2},
            {"datetime": "2023-01-01 11:00:00", "value": 3},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        # Chain multiple operations
        result = frame.dt_year("datetime").dt_month("datetime").dt_day("datetime")
        
        assert "datetime_year" in result.columns
        assert "datetime_month" in result.columns
        assert "datetime_day" in result.columns
        assert result.len() == 3
        
        years = self.get_column_data(result, "datetime_year")
        months = self.get_column_data(result, "datetime_month")
        days = self.get_column_data(result, "datetime_day")
        
        assert years == [2023, 2023, 2023]
        assert months == [1, 1, 1]
        assert days == [1, 1, 1]

    def test_time_series_large_dataset(self):
        """Test time series operations with larger dataset"""
        data = []
        base_time = "2023-01-01 10:00:00"
        for i in range(100):
            data.append({
                "datetime": f"2023-01-{(i % 30) + 1:02d} 10:00:00",
                "value": i
            })
        
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_year("datetime")
        
        assert "datetime_year" in result.columns
        assert result.len() == 100
        
        years = self.get_column_data(result, "datetime_year")
        assert all(year == 2023 for year in years)

    def test_time_series_invalid_datetime_format(self):
        """Test time series operations with invalid datetime format"""
        data = [
            {"datetime": "invalid-date", "value": 1},
            {"datetime": "2023-01-01 10:00:00", "value": 2},
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_year("datetime")
        
        assert "datetime_year" in result.columns
        assert result.len() == 2
        
        years = self.get_column_data(result, "datetime_year")
        assert years[0] == 0  # Invalid date should result in 0
        assert years[1] == 2023

    def test_time_series_edge_cases(self):
        """Test time series operations with edge cases"""
        data = [
            {"datetime": "2023-02-29 10:00:00", "value": 1},  # Invalid date (2023 is not leap year)
            {"datetime": "2023-01-01 00:00:00", "value": 2},  # Midnight
            {"datetime": "2023-12-31 23:59:59", "value": 3},  # End of year
        ]
        frame = ft.TinyFrame.from_dicts(data)
        
        result = frame.dt_year("datetime")
        
        assert "datetime_year" in result.columns
        assert result.len() == 3
        
        years = self.get_column_data(result, "datetime_year")
        assert years[0] == 0  # Invalid date should result in 0
        assert years[1] == 2023
        assert years[2] == 2023
