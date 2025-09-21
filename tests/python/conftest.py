"""
Pytest configuration and fixtures for feathertail tests.
"""

import pytest
import feathertail as ft
import pandas as pd
import numpy as np
from typing import List, Dict, Any


@pytest.fixture
def sample_records() -> List[Dict[str, Any]]:
    """Sample data for testing."""
    return [
        {"name": "Alice", "age": 30, "city": "New York", "score": 95.5},
        {"name": "Bob", "age": 25, "city": "Paris", "score": 85.0},
        {"name": "Charlie", "age": 35, "city": "London", "score": 78.5},
        {"name": "Diana", "age": None, "city": "Tokyo", "score": 92.0},
        {"name": "Eve", "age": 28, "city": None, "score": None},
    ]


@pytest.fixture
def sample_frame(sample_records):
    """Sample TinyFrame for testing."""
    return ft.TinyFrame.from_dicts(sample_records)


@pytest.fixture
def sample_pandas_frame(sample_records):
    """Sample pandas DataFrame for comparison testing."""
    return pd.DataFrame(sample_records)


@pytest.fixture
def large_records() -> List[Dict[str, Any]]:
    """Large dataset for performance testing."""
    np.random.seed(42)
    return [
        {
            "id": i,
            "value": np.random.randn(),
            "category": np.random.choice(["A", "B", "C", "D", "E"]),
            "text": f"text_{i}",
        }
        for i in range(1000)
    ]


@pytest.fixture
def mixed_type_records() -> List[Dict[str, Any]]:
    """Records with mixed data types for type inference testing."""
    return [
        {"mixed": 42, "other": "hello"},
        {"mixed": "world", "other": 3.14},
        {"mixed": True, "other": None},
        {"mixed": None, "other": [1, 2, 3]},
    ]


@pytest.fixture
def empty_records() -> List[Dict[str, Any]]:
    """Empty records for edge case testing."""
    return []


@pytest.fixture
def single_record() -> List[Dict[str, Any]]:
    """Single record for edge case testing."""
    return [{"name": "Alice", "age": 30}]
