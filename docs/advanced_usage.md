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
# Many operations automatically use parallel processing
# GroupBy operations are parallelized
grouped = df.groupby("category").agg({"value": ["mean", "sum", "std"]})

# Large joins are parallelized
large_join = df1.inner_join(df2, ["key1", "key2"], ["key1", "key2"])

# Sorting large datasets uses parallel algorithms
sorted_df = df.sort_values(["col1", "col2"], ascending=[True, False])
```

### Chunked Processing

```python
# For very large datasets, process in chunks
def process_chunk(chunk_data):
    df = ft.TinyFrame.from_dicts(chunk_data)
    return df.groupby("category").agg({"value": "sum"})

# Process data in chunks
chunk_size = 10000
results = []
for i in range(0, len(large_data), chunk_size):
    chunk = large_data[i:i + chunk_size]
    result = process_chunk(chunk)
    results.append(result)

# Combine results
final_result = ft.TinyFrame.concat(results)
```

## Advanced Analytics

### Custom Aggregations

```python
# Use multiple aggregation functions
df = ft.TinyFrame.from_dicts([
    {"category": "A", "value": 10, "count": 1},
    {"category": "A", "value": 20, "count": 2},
    {"category": "B", "value": 30, "count": 1},
    {"category": "B", "value": 40, "count": 3}
])

# Multiple aggregations
result = df.groupby("category").agg({
    "value": ["sum", "mean", "std", "min", "max"],
    "count": ["sum", "count"]
})
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

# Quantile analysis
quantiles = df.quantile([0.25, 0.5, 0.75])

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

### Time Differences and Shifting

```python
# Calculate time differences
df = df.dt_diff("timestamp")

# Shift timestamps
df = df.dt_shift("timestamp", hours=1)
df = df.dt_shift("timestamp", days=7)
```

### Rolling and Expanding Windows

```python
# Rolling window operations
df = df.rolling_mean("value", window=5)
df = df.rolling_sum("value", window=10)
df = df.rolling_std("value", window=7)

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
    # Operation that might fail
    result = df.groupby("nonexistent_column").agg({"value": "mean"})
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
