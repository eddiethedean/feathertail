# Getting Started with Feathertail

Feathertail is a high-performance Python DataFrame library powered by Rust, designed to provide pandas-like functionality with superior performance and memory efficiency.

## Installation

```bash
pip install feathertail
```

## Quick Start

### Creating a DataFrame

```python
import feathertail as ft

# From a list of dictionaries
data = [
    {"name": "Alice", "age": 25, "city": "New York"},
    {"name": "Bob", "age": 30, "city": "San Francisco"},
    {"name": "Charlie", "age": 35, "city": "Chicago"}
]
df = ft.TinyFrame.from_dicts(data)

# From a pandas DataFrame
import pandas as pd
pandas_df = pd.DataFrame(data)
df = ft.TinyFrame.from_pandas(pandas_df)
```

### Basic Operations

```python
# View data
print(df.head())
print(df.info())

# Basic statistics
print(df.describe())

# Data shape
print(f"Shape: {df.shape}")
print(f"Columns: {df.columns}")
```

### Data Manipulation

```python
# Filtering
filtered = df.filter("age", ">", 25)
print(filtered)

# Sorting
sorted_df = df.sort_values("age", ascending=False)
print(sorted_df)

# Handling missing values
df_with_nulls = df.fillna({"age": 0})
cleaned = df_with_nulls.dropna()
```

### GroupBy Operations

```python
# Group by city and calculate mean age
grouped = df.groupby("city").agg({"age": "mean"})
print(grouped)

# Multiple aggregations
multi_agg = df.groupby("city").agg({
    "age": ["mean", "max", "min"],
    "name": "count"
})
print(multi_agg)
```

### Join Operations

```python
# Create another DataFrame
scores = ft.TinyFrame.from_dicts([
    {"name": "Alice", "score": 95},
    {"name": "Bob", "score": 87},
    {"name": "Charlie", "score": 92}
])

# Inner join
joined = df.inner_join(scores, ["name"], ["name"])
print(joined)
```

### Analytics Functions

```python
# Descriptive statistics
stats = df.describe()
print(stats)

# Correlation
correlation = df.corr()
print(correlation)

# Skewness and kurtosis
skewness = df.skew()
kurtosis = df.kurtosis()
```

### Time Series Operations

```python
# Create time series data
time_data = [
    {"timestamp": "2023-01-01 10:00:00", "value": 100},
    {"timestamp": "2023-01-01 11:00:00", "value": 110},
    {"timestamp": "2023-01-01 12:00:00", "value": 105}
]
ts_df = ft.TinyFrame.from_dicts(time_data)

# Convert to timestamps
ts_df = ts_df.to_timestamps("timestamp")

# Extract time components
ts_df = ts_df.dt_year("timestamp")
ts_df = ts_df.dt_month("timestamp")
ts_df = ts_df.dt_hour("timestamp")
```

### String Operations

```python
# String manipulation
df = df.str_upper("name")
df = df.str_strip("city")
df = df.str_replace("city", "New York", "NYC")

# Pattern matching
contains_ny = df.str_contains("city", "NY")
```

### Data Validation

```python
# Validate data quality
df = df.validate_not_null("age")
df = df.validate_range("age", min=0, max=120)
df = df.validate_unique("name")

# Get validation summary
summary = df.validation_summary("age")
print(summary)
```

## Performance Tips

1. **Use appropriate data types**: Feathertail automatically infers types, but you can cast columns for better performance.

2. **Batch operations**: Chain operations together for better performance.

3. **Memory management**: Use `dropna()` and `fillna()` to manage memory usage.

4. **Parallel processing**: Many operations automatically use parallel processing for large datasets.

## Migration from Pandas

Feathertail is designed to be compatible with pandas. Here are the main differences:

| Pandas | Feathertail | Notes |
|--------|-------------|-------|
| `pd.DataFrame()` | `ft.TinyFrame.from_dicts()` | Constructor |
| `df.head()` | `df.head()` | Same |
| `df.describe()` | `df.describe()` | Same |
| `df.groupby()` | `df.groupby()` | Same |
| `df.merge()` | `df.inner_join()` | Different method name |
| `df.str.upper()` | `df.str_upper()` | Different method name |

## Next Steps

- Check out the [Advanced Usage Guide](advanced_usage.md) for more complex operations
- Explore the [API Reference](api/index.md) for complete documentation
- Try the [Tutorial Series](tutorials/index.md) for hands-on examples
