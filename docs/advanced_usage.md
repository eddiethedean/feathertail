# Advanced Usage Guide

This guide covers advanced features and optimization techniques in feathertail.

## Performance Optimization

### Memory Management

```python
import feathertail as ft

# Use appropriate data types
data = [{"id": i, "value": float(i)} for i in range(1000000)]
df = ft.TinyFrame.from_dicts(data)

# Cast columns to appropriate types for better performance
df = df.cast_column("id", "int")
df = df.cast_column("value", "float")

# Remove unnecessary columns
df = df.drop_columns(["unused_col"])

# Use fillna instead of keeping nulls when possible
df = df.fillna({"value": 0.0})
```

### Parallel Processing

```python
# Internal parallel paths apply to sorting and filtering on large frames.
# High-level TinyGroupBy uses the API shown above (string grouping columns).
gb = ft.TinyGroupBy(df, ["category"])
grouped_sum = gb.sum(df, "value")

large_join = df1.inner_join(df2, ["key1", "key2"], ["key1", "key2"])

sorted_df = df.sort_values(["col1", "col2"], ascending=[True, False])
```

### Chunked Processing

```python
# For very large datasets, process logical chunks in your pipeline (each chunk is a TinyFrame).
def process_chunk(chunk_records):
    df = ft.TinyFrame.from_dicts(chunk_records)
    return ft.TinyGroupBy(df, ["category"]).sum(df, "value")

chunk_size = 10000
aggregates = []
for i in range(0, len(large_data), chunk_size):
    chunk = large_data[i:i + chunk_size]
    aggregates.append(process_chunk(chunk))

# Combine downstream by merging dict rows or joining aggregated TinyFrames as your app requires.
```

## Advanced Analytics

### Custom Aggregations

```python
df = ft.TinyFrame.from_dicts([
    {"category": "A", "value": 10, "count": 1},
    {"category": "A", "value": 20, "count": 2},
    {"category": "B", "value": 30, "count": 1},
    {"category": "B", "value": 40, "count": 3}
])

gb = ft.TinyGroupBy(df, ["category"])
sum_values = gb.sum(df, "value")
mean_values = gb.mean(df, "value")
sum_counts = gb.sum(df, "count")
```

### Correlation Analysis

```python
# Calculate correlation matrix
correlation_matrix = df.corr()

# Calculate correlation with specific column
corr_with_target = df.corr_with("target_column")

# Calculate covariance
covariance_matrix = df.cov()
```

### Statistical Analysis

```python
# Comprehensive descriptive statistics
stats = df.describe()

# Distribution analysis
skewness = df.skew()
kurtosis = df.kurtosis()

# Quantile for one probability per call (e.g. median ≈ 0.5)
q50 = df.quantile("value_column", 0.5)

# Mode calculation
modes = df.mode()

# Unique value counting
unique_counts = df.nunique()
```

## Time Series Analysis

### Time Component Extraction

```python
# Create time series data
time_data = [
    {"timestamp": "2023-01-01 10:30:00", "value": 100},
    {"timestamp": "2023-01-01 11:45:00", "value": 110},
    {"timestamp": "2023-01-02 09:15:00", "value": 105}
]
df = ft.TinyFrame.from_dicts(time_data)

# Convert to timestamps
df = df.to_timestamps("timestamp")

# Extract all time components
df = df.dt_year("timestamp")
df = df.dt_month("timestamp")
df = df.dt_day("timestamp")
df = df.dt_hour("timestamp")
df = df.dt_minute("timestamp")
df = df.dt_second("timestamp")
df = df.dt_day_of_week("timestamp")
df = df.dt_day_of_year("timestamp")
```

`to_timestamps`, `dt_year`, `dt_month`, and related methods parse strings **strictly**: whitespace-only cells, unrecognized formats, and impossible calendar dates raise `ValueError`. For optional string columns, missing values (`None`) use sentinel `0` in extracted components; `to_timestamps` on optional strings produces an optional integer timestamp column.

### Time Differences and Shifting

```python
# Calculate time differences
df = df.dt_diff("timestamp")

# Shift datetime strings by a delta in seconds (e.g. +1 hour = 3600)
df = df.dt_shift("timestamp", 3600)
df = df.dt_shift("timestamp", 7 * 24 * 3600)  # add 7 days
```

### Rolling and Expanding Windows

```python
# Rolling window operations (window size is the second positional argument)
df = df.rolling_mean("value", 5)
df = df.rolling_sum("value", 10)
df = df.rolling_std("value", 7)

# Expanding window operations
df = df.expanding_mean("value")
df = df.expanding_sum("value")
```

## Advanced String Operations

### Pattern Matching and Extraction

```python
# String case operations
df = df.str_upper("text_column")
df = df.str_lower("text_column")

# Whitespace handling
df = df.str_strip("text_column")

# String replacement
df = df.str_replace("text_column", "old_pattern", "new_pattern")

# String splitting
df = df.str_split("text_column", delimiter=",")

# Pattern matching
df = df.str_contains("text_column", "pattern")

# String length
df = df.str_len("text_column")

# String concatenation
df = df.str_cat("text_column", separator=", ")
```

### Regular Expression Operations

```python
# Pattern validation
df = df.validate_pattern("email", r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")

# String extraction using patterns
# (Note: This would be implemented as an extension)
```

## Data Validation and Quality

### Comprehensive Validation

```python
# Null value validation
df = df.validate_not_null("required_column")

# Range validation
df = df.validate_range("age", min=0, max=120)
df = df.validate_range("score", min=0, max=100)

# Pattern validation
df = df.validate_pattern("email", r"^[^@]+@[^@]+\.[^@]+$")
df = df.validate_pattern("phone", r"^\d{3}-\d{3}-\d{4}$")

# Uniqueness validation
df = df.validate_unique("id")
df = df.validate_unique("email")
```

### Validation Summary

```python
# Get comprehensive validation summary
summary = df.validation_summary("column_name")
print(f"Total count: {summary['total_count']}")
print(f"Null count: {summary['null_count']}")
print(f"Null percentage: {summary['null_percentage']:.2f}%")
print(f"Non-null count: {summary['non_null_count']}")
```

## Advanced Join Operations

### Multiple Column Joins

```python
# Join on multiple columns
df1 = ft.TinyFrame.from_dicts([
    {"id": 1, "category": "A", "value": 10},
    {"id": 2, "category": "B", "value": 20}
])

df2 = ft.TinyFrame.from_dicts([
    {"id": 1, "category": "A", "score": 95},
    {"id": 2, "category": "B", "score": 87}
])

# Join on multiple columns
joined = df1.inner_join(df2, ["id", "category"], ["id", "category"])
```

### Different Join Types

```python
# Inner join (only matching rows)
inner = df1.inner_join(df2, ["key"], ["key"])

# Left join (all rows from left, matching from right)
left = df1.left_join(df2, ["key"], ["key"])

# Right join (all rows from right, matching from left)
right = df1.right_join(df2, ["key"], ["key"])

# Outer join (all rows from both)
outer = df1.outer_join(df2, ["key"], ["key"])

# Cross join (cartesian product)
cross = df1.cross_join(df2)
```

## Error Handling and Debugging

### Common Error Patterns

```python
try:
    # Operation that might fail (group key column missing)
    gb = ft.TinyGroupBy(df, ["nonexistent_column"])
    result = gb.mean(df, "value")
except KeyError as e:
    print(f"Column not found: {e}")
    # Handle error appropriately

try:
    # Type conversion that might fail
    df = df.cast_column("text_column", "int")
except TypeError as e:
    print(f"Type conversion failed: {e}")
    # Handle error appropriately
```

### Debugging Tips

```python
# Check DataFrame structure
print(df.info())
print(df.dtypes)

# Check for null values
print(df.isnull().sum())

# Validate data before operations
validation_result = df.validate_not_null("critical_column")
if not validation_result["critical_column_not_null"].all():
    print("Warning: Critical column has null values")
```

## Best Practices

1. **Type Safety**: Always use appropriate data types for your columns
2. **Memory Management**: Clean up unused columns and handle nulls appropriately
3. **Error Handling**: Always handle potential errors gracefully
4. **Validation**: Validate data quality before performing operations
5. **Performance**: Use parallel operations and appropriate data types for large datasets
6. **Documentation**: Document your data processing pipeline clearly

## Integration with Other Libraries

### Pandas Integration

```python
import pandas as pd
import feathertail as ft

# Convert from pandas
pandas_df = pd.DataFrame(data)
feathertail_df = ft.TinyFrame.from_pandas(pandas_df)

# Convert to pandas
result_pandas = feathertail_df.to_pandas()
```

### NumPy Integration

```python
import numpy as np

# Convert to numpy arrays
values = df["value"].to_numpy()
categories = df["category"].to_numpy()
```

This concludes the advanced usage guide. For more specific examples, check out the tutorial series.
