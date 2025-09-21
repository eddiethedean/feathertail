# Tutorial 1: Feathertail Basics

This tutorial introduces you to the fundamental concepts of feathertail and shows you how to perform basic operations.

## What You'll Learn

- How to create DataFrames
- Basic data inspection
- Simple data manipulation
- Understanding data types

## Prerequisites

- Python 3.7+
- Basic knowledge of Python
- feathertail installed (`pip install feathertail`)

## Creating Your First DataFrame

### From a List of Dictionaries

```python
import feathertail as ft

# Sample data
data = [
    {"name": "Alice", "age": 25, "city": "New York", "salary": 75000},
    {"name": "Bob", "age": 30, "city": "San Francisco", "salary": 95000},
    {"name": "Charlie", "age": 35, "city": "Chicago", "salary": 85000},
    {"name": "Diana", "age": 28, "city": "New York", "salary": 80000},
    {"name": "Eve", "age": 32, "city": "San Francisco", "salary": 90000}
]

# Create DataFrame
df = ft.TinyFrame.from_dicts(data)
print("DataFrame created successfully!")
```

### From a Pandas DataFrame

```python
import pandas as pd

# Create pandas DataFrame
pandas_df = pd.DataFrame(data)

# Convert to feathertail
df = ft.TinyFrame.from_pandas(pandas_df)
print("Converted from pandas DataFrame")
```

## Inspecting Your Data

### Basic Information

```python
# View the first few rows
print("First 3 rows:")
print(df.head(3))

# View the last few rows
print("\nLast 2 rows:")
print(df.tail(2))

# Get basic information
print("\nDataFrame info:")
print(df.info())

# Get shape
print(f"\nShape: {df.shape}")
print(f"Rows: {df.shape[0]}, Columns: {df.shape[1]}")

# Get column names
print(f"\nColumns: {df.columns}")
```

### Data Types

```python
# Check data types
print("Data types:")
print(df.dtypes)

# Check for null values
print("\nNull values:")
print(df.isnull().sum())
```

## Basic Operations

### Selecting Data

```python
# Get a specific column
ages = df["age"]
print("Ages:", ages)

# Get multiple columns
subset = df[["name", "age"]]
print("\nName and age columns:")
print(subset)
```

### Filtering Data

```python
# Filter by age
young_employees = df.filter("age", "<", 30)
print("Employees under 30:")
print(young_employees)

# Filter by city
ny_employees = df.filter("city", "==", "New York")
print("\nNew York employees:")
print(ny_employees)

# Multiple conditions
high_earners = df.filter("salary", ">", 80000).filter("age", ">", 25)
print("\nHigh earners over 25:")
print(high_earners)
```

### Sorting Data

```python
# Sort by age (ascending)
sorted_by_age = df.sort_values("age")
print("Sorted by age (ascending):")
print(sorted_by_age)

# Sort by salary (descending)
sorted_by_salary = df.sort_values("salary", ascending=False)
print("\nSorted by salary (descending):")
print(sorted_by_salary)

# Sort by multiple columns
multi_sorted = df.sort_values(["city", "salary"], ascending=[True, False])
print("\nSorted by city (asc), then salary (desc):")
print(multi_sorted)
```

## Data Manipulation

### Adding New Columns

```python
# Add a calculated column
df_with_bonus = df.copy()
df_with_bonus["bonus"] = df_with_bonus["salary"] * 0.1
print("DataFrame with bonus column:")
print(df_with_bonus)
```

### Handling Missing Data

```python
# Create data with missing values
data_with_nulls = [
    {"name": "Alice", "age": 25, "city": "New York", "salary": 75000},
    {"name": "Bob", "age": None, "city": "San Francisco", "salary": 95000},
    {"name": "Charlie", "age": 35, "city": None, "salary": 85000},
    {"name": "Diana", "age": 28, "city": "New York", "salary": None}
]

df_with_nulls = ft.TinyFrame.from_dicts(data_with_nulls)
print("DataFrame with missing values:")
print(df_with_nulls)

# Fill missing values
filled_df = df_with_nulls.fillna({"age": 0, "city": "Unknown", "salary": 0})
print("\nAfter filling missing values:")
print(filled_df)

# Drop rows with any missing values
clean_df = df_with_nulls.dropna()
print("\nAfter dropping rows with missing values:")
print(clean_df)
```

### Type Conversion

```python
# Convert column types
df_converted = df.cast_column("age", "float")
print("Age column converted to float:")
print(df_converted.dtypes)

# Convert back to int
df_converted = df_converted.cast_column("age", "int")
print("\nAge column converted back to int:")
print(df_converted.dtypes)
```

## Basic Statistics

### Descriptive Statistics

```python
# Get basic statistics
stats = df.describe()
print("Descriptive statistics:")
print(stats)

# Get statistics for specific columns
age_stats = df["age"].describe()
print("\nAge statistics:")
print(age_stats)
```

### Simple Calculations

```python
# Calculate mean age
mean_age = df["age"].mean()
print(f"Mean age: {mean_age}")

# Calculate total salary
total_salary = df["salary"].sum()
print(f"Total salary: ${total_salary:,}")

# Count employees by city
city_counts = df["city"].value_counts()
print(f"\nEmployees by city:")
print(city_counts)
```

## Converting Back to Other Formats

### To Python Dictionary

```python
# Convert to list of dictionaries
dict_data = df.to_dicts()
print("Converted to list of dictionaries:")
print(dict_data[:2])  # Show first 2 rows
```

### To Pandas DataFrame

```python
# Convert to pandas DataFrame
pandas_df = df.to_pandas()
print("Converted to pandas DataFrame:")
print(pandas_df.head())
print(f"Pandas DataFrame type: {type(pandas_df)}")
```

## Exercise

Try these exercises to practice what you've learned:

1. **Create a DataFrame** with information about your favorite books (title, author, year, rating).

2. **Filter the data** to show only books with a rating above 4.0.

3. **Sort the data** by year in descending order.

4. **Add a new column** that categorizes books as "Old" (before 2000) or "New" (2000 and later).

5. **Calculate statistics** for the rating column.

## Next Steps

- [Tutorial 2: Data Manipulation](02_data_manipulation.md) - Learn about advanced filtering and data cleaning
- [Tutorial 3: GroupBy and Aggregations](03_groupby.md) - Learn how to group data and perform aggregations
- [Getting Started Guide](../getting_started.md) - Quick reference for common operations
- [API Reference](../api/index.md) - Complete documentation of all methods
